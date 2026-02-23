/// デーモンモード — Unix ドメインソケット経由の常駐ランタイム。
///
/// プロトコル（改行区切り JSON）:
///   クライアント → サーバー: `{"lang":"py","script":"a.py","args":["x"]}`
///   サーバー → クライアント: `{"exit":0,"stdout":"...","stderr":"..."}`
///   停止要求:               `{"lang":"","script":"","stop":true}`
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};

const SOCK: &str = "/tmp/polyscript_daemon.sock";
const PID_FILE: &str = "/tmp/polyscript_daemon.pid";

#[derive(Serialize, Deserialize)]
struct Req {
    lang: String,
    script: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    stop: bool,
}

#[derive(Serialize, Deserialize)]
struct Resp {
    exit: i32,
    stdout: String,
    stderr: String,
}

/// `polyscript daemon start` — 自分自身を `daemon serve` モードでバックグラウンド起動。
pub fn start() -> Result<()> {
    let exe = std::env::current_exe()?;
    let child = std::process::Command::new(exe)
        .args(["daemon", "serve"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    std::fs::write(PID_FILE, child.id().to_string())?;
    println!("polyscript daemon started (PID {})", child.id());
    Ok(())
}

/// `polyscript daemon serve` — Unix ソケットをリッスンするサーバーループ（内部用）。
pub fn serve() -> Result<()> {
    let _ = std::fs::remove_file(SOCK);
    let listener = UnixListener::bind(SOCK)?;
    eprintln!("[polyscript daemon] listening on {SOCK}");
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            if let Err(e) = handle_conn(stream) {
                eprintln!("[polyscript daemon] connection error: {e}");
            }
        });
    }
    Ok(())
}

fn handle_conn(mut stream: UnixStream) -> Result<()> {
    let exe = std::env::current_exe()?;
    let reader = BufReader::new(stream.try_clone()?);
    for line in reader.lines() {
        let req: Req = serde_json::from_str(&line?)?;
        if req.stop {
            writeln!(stream, r#"{{"exit":0,"stdout":"","stderr":"daemon stopped"}}"#)?;
            // 全スレッドを安全に終了
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(100));
                std::process::exit(0);
            });
            return Ok(());
        }
        // 各リクエストは polyscript 自身を subprocess として実行（全ブリッジを再利用）
        let out = std::process::Command::new(&exe)
            .arg(&req.lang)
            .arg(&req.script)
            .args(&req.args)
            .output()?;
        let resp = Resp {
            exit: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        };
        writeln!(stream, "{}", serde_json::to_string(&resp)?)?;
    }
    Ok(())
}

/// `polyscript daemon run` — デーモン経由でスクリプトを実行（クライアント側）。
pub fn run_via(lang: &str, script: &str, args: &[String]) -> Result<()> {
    let req = Req { lang: lang.into(), script: script.into(), args: args.to_vec(), stop: false };
    let mut stream = UnixStream::connect(SOCK)
        .map_err(|_| anyhow::anyhow!("daemon not running — start with `polyscript daemon start`"))?;
    writeln!(stream, "{}", serde_json::to_string(&req)?)?;
    stream.shutdown(std::net::Shutdown::Write)?;
    for line in BufReader::new(stream).lines() {
        let resp: Resp = serde_json::from_str(&line?)?;
        print!("{}", resp.stdout);
        eprint!("{}", resp.stderr);
        anyhow::ensure!(resp.exit == 0, "script exited with {}", resp.exit);
    }
    Ok(())
}

/// `polyscript daemon stop` — デーモンへ停止シグナルを送る（クライアント側）。
pub fn stop() -> Result<()> {
    let req = Req { lang: "".into(), script: "".into(), args: vec![], stop: true };
    let mut stream = UnixStream::connect(SOCK)
        .map_err(|_| anyhow::anyhow!("daemon not running"))?;
    writeln!(stream, "{}", serde_json::to_string(&req)?)?;
    let mut buf = String::new();
    BufReader::new(stream).read_line(&mut buf)?;
    println!("daemon stopped");
    Ok(())
}
