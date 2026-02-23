/// Julia ブリッジ — juliac AOT コンパイル版。Julia 1.12+ の juliac が必要。
/// `juliac --output-exe /tmp/polyscript_julia_<pid> <script>` でネイティブバイナリを生成し実行。
pub fn run(s: &str, a: &[String]) -> anyhow::Result<()> {
    use anyhow::ensure;
    let out = format!("/tmp/polyscript_julia_{}", std::process::id());
    ensure!(
        std::process::Command::new("juliac")
            .args(["--output-exe", &out])
            .arg(s)
            .status()?
            .success(),
        "juliac: compilation failed"
    );
    ensure!(
        std::process::Command::new(&out).args(a).status()?.success(),
        "julia binary exited non-zero"
    );
    Ok(())
}
