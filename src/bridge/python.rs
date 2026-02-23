use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;

/// Run a Python script file via PyO3 FFI bridge.
pub fn run(script: &str, args: &[String]) -> Result<()> {
    let code = fs::read_to_string(script)?;

    Python::with_gil(|py| {
        let sys = py.import_bound("sys")?;
        let argv: Vec<&str> = std::iter::once(script)
            .chain(args.iter().map(|s| s.as_str()))
            .collect();
        sys.setattr("argv", PyList::new_bound(py, &argv))?;

        // インプロセス実行時は os.environ にも POLYSCRIPT_IPC_PATH を注入する
        // （subprocess と違い親プロセスの set_var が Python os.environ に自動反映されないため）
        if let Ok(ipc) = std::env::var("POLYSCRIPT_IPC_PATH") {
            py.import_bound("os")?
                .getattr("environ")?
                .set_item("POLYSCRIPT_IPC_PATH", ipc)?;
        }

        py.run_bound(&code, None, None)?;
        Ok(())
    })
}
