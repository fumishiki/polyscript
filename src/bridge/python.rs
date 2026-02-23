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
        let py_args = PyList::new_bound(py, &argv);
        sys.setattr("argv", py_args)?;
        py.run_bound(&code, None, None)?;
        Ok(())
    })
}
