use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Deserialize;
use std::collections::HashMap;

mod bridge;
mod daemon;
use bridge::*;

// ── CLI ──────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "polyscript", about = "Polyglot script runner — 16 languages via FFI / subprocess")]
struct Cli {
    /// IPC フォーマット。設定すると POLYSCRIPT_IPC_PATH=/tmp/polyscript_ipc_<pid>.<ext> を
    /// 自動生成し、サブプロセスへ環境変数として伝播する。
    #[arg(long, value_enum, global = true)]
    ipc_format: Option<IpcFormat>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Clone, ValueEnum)]
enum IpcFormat { Arrow, Parquet, Json }

impl IpcFormat {
    fn ext(&self) -> &'static str {
        match self { Self::Arrow => "arrow", Self::Parquet => "parquet", Self::Json => "json" }
    }
}

// ── 共有引数 / サブコマンド ────────────────────────────────────────────────

#[derive(Args)]
struct S {
    /// スクリプトファイルのパス
    script: String,
    /// スクリプトへ渡す引数
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Python スクリプト（PyO3 インプロセス）
    Py(S),
    /// Julia スクリプト（juliac AOT コンパイル）
    Jl(S),
    /// Go スクリプト（go run）
    Go(S),
    /// JavaScript（node）
    Js(S),
    /// TypeScript（deno run）
    Ts(S),
    /// Lua スクリプト
    Lua(S),
    /// R スクリプト（Rscript）
    R(S),
    /// Mojo スクリプト
    Mojo(S),
    /// Zig スクリプト（zig run）
    Zig(S),
    /// WebAssembly モジュール（wasmtime run）
    Wasm(S),
    /// Haskell スクリプト（runghc）
    Hs(S),
    /// Swift スクリプト
    Swift(S),
    /// Kotlin スクリプト（kotlinc -script）
    Kt(S),
    /// Kotlin AOT（kotlinc compile → java -jar）
    Ktn(S),
    /// Nim スクリプト（nim r）
    Nim(S),
    /// Fortran ソース（gfortran compile + run）
    Fort(S),
    /// C++ 共有ライブラリ関数（libloading）
    Cpp { lib: String, func: String, args: Vec<String> },
    /// polyscript.toml のエイリアスを実行
    Run {
        name: String,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// 複数スペックを並列実行: "py a.py x" "jl b.jl y"
    Parallel {
        #[arg(trailing_var_arg = true)]
        specs: Vec<String>,
    },
    /// デーモンモード（Unix ソケット常駐ランタイム）
    Daemon {
        #[command(subcommand)]
        cmd: DaemonCmd,
    },
}

#[derive(Subcommand)]
enum DaemonCmd {
    /// デーモンをバックグラウンドで起動
    Start,
    /// サーバーループ（内部用 — 直接呼び出し不要）
    Serve,
    /// デーモン経由でスクリプトを実行: <lang> <script> [args...]
    Run { lang: String, script: String, args: Vec<String> },
    /// デーモンを停止
    Stop,
}

// ── polyscript.toml ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct PolyConfig {
    scripts: HashMap<String, ScriptEntry>,
}

#[derive(Deserialize)]
struct ScriptEntry {
    lang: String,
    script: String,
}

// ── 言語ディスパッチャ ────────────────────────────────────────────────────

fn dispatch_lang(lang: &str, script: &str, args: &[String]) -> Result<()> {
    match lang {
        "py"    => python::run(script, args),
        "jl"    => julia::run(script, args),
        "go"    => go::run(script, args),
        "js"    => js::run(script, args),
        "ts"    => ts::run(script, args),
        "lua"   => lua::run(script, args),
        "r"     => r::run(script, args),
        "mojo"  => mojo::run(script, args),
        "zig"   => zig::run(script, args),
        "wasm"  => wasm::run(script, args),
        "hs"    => hs::run(script, args),
        "swift" => swift::run(script, args),
        "kt"    => kt::run(script, args),
        "ktn"   => ktn::run(script, args),
        "nim"   => nim::run(script, args),
        "fort"  => fort::run(script, args),
        _ => bail!("unknown language: {lang}"),
    }
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    use Cmd::*;
    let cli = Cli::parse();

    // IPC パス生成 → POLYSCRIPT_IPC_PATH をサブプロセスへ伝播
    if let Some(ref fmt) = cli.ipc_format {
        let path = format!("/tmp/polyscript_ipc_{}.{}", std::process::id(), fmt.ext());
        // SAFETY: スレッド生成前のシングルスレッド文脈
        unsafe { std::env::set_var("POLYSCRIPT_IPC_PATH", &path) }
        eprintln!("[polyscript] POLYSCRIPT_IPC_PATH={path}");
    }

    match cli.cmd {
        Py(a)    => python::run(&a.script, &a.args),
        Jl(a)    => julia::run(&a.script, &a.args),
        Go(a)    => go::run(&a.script, &a.args),
        Js(a)    => js::run(&a.script, &a.args),
        Ts(a)    => ts::run(&a.script, &a.args),
        Lua(a)   => lua::run(&a.script, &a.args),
        R(a)     => r::run(&a.script, &a.args),
        Mojo(a)  => mojo::run(&a.script, &a.args),
        Zig(a)   => zig::run(&a.script, &a.args),
        Wasm(a)  => wasm::run(&a.script, &a.args),
        Hs(a)    => hs::run(&a.script, &a.args),
        Swift(a) => swift::run(&a.script, &a.args),
        Kt(a)    => kt::run(&a.script, &a.args),
        Ktn(a)   => ktn::run(&a.script, &a.args),
        Nim(a)   => nim::run(&a.script, &a.args),
        Fort(a)  => fort::run(&a.script, &a.args),
        Cpp { lib, func, args } => cpp::run(&lib, &func, &args),

        Run { name, args } => {
            let toml_str = std::fs::read_to_string("polyscript.toml")
                .map_err(|_| anyhow::anyhow!("polyscript.toml not found in current directory"))?;
            let cfg: PolyConfig = toml::from_str(&toml_str)?;
            let e = cfg.scripts.get(&name)
                .ok_or_else(|| anyhow::anyhow!("unknown alias: {name}"))?;
            dispatch_lang(&e.lang, &e.script, &args)
        }

        Parallel { specs } => {
            let handles: Vec<_> = specs.into_iter().map(|spec| {
                std::thread::spawn(move || -> Result<()> {
                    let mut p = spec.split_whitespace();
                    let lang   = p.next().ok_or_else(|| anyhow::anyhow!("empty spec"))?.to_owned();
                    let script = p.next().ok_or_else(|| anyhow::anyhow!("missing script"))?.to_owned();
                    let args: Vec<String> = p.map(String::from).collect();
                    // PyO3 はスレッド間で GIL を競合するため subprocess にフォールバック
                    if lang == "py" {
                        bridge::sp("python3", &[], &script, &args)
                    } else {
                        dispatch_lang(&lang, &script, &args)
                    }
                })
            }).collect();
            for h in handles {
                h.join().map_err(|e| anyhow::anyhow!("thread panicked: {e:?}"))??;
            }
            Ok(())
        }

        Daemon { cmd } => match cmd {
            DaemonCmd::Start                        => daemon::start(),
            DaemonCmd::Serve                        => daemon::serve(),
            DaemonCmd::Run { lang, script, args }   => daemon::run_via(&lang, &script, &args),
            DaemonCmd::Stop                         => daemon::stop(),
        },
    }
}

