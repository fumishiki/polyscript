use anyhow::{bail, Result};
use std::process::Command;

/// Run a Julia script via subprocess (libjulia FFI is optional).
/// To use native FFI, add libjulia-sys and replace this stub.
pub fn run(script: &str, args: &[String]) -> Result<()> {
    let status = Command::new("julia")
        .arg(script)
        .args(args)
        .status()?;

    if !status.success() {
        bail!("Julia exited with {}", status);
    }
    Ok(())
}
