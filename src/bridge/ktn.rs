use anyhow::{Result, ensure};
use std::process::Command;

/// Kotlin AOT ブリッジ — kotlinc で fat JAR にコンパイルし java -jar で実行。
/// `kt`（kotlinc -script）の scripting overhead を回避し、JVM 起動のみのコストに抑える。
pub fn run(s: &str, a: &[String]) -> Result<()> {
    let jar = format!("/tmp/polyscript_kt_{}.jar", std::process::id());
    ensure!(
        Command::new("kotlinc")
            .arg(s)
            .args(["-include-runtime", "-d", &jar])
            .status()?
            .success(),
        "kotlinc: compilation failed"
    );
    ensure!(
        Command::new("java")
            .args(["-jar", &jar])
            .args(a)
            .status()?
            .success(),
        "java -jar: non-zero exit"
    );
    Ok(())
}
