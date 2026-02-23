use anyhow::{bail, Result};
use libloading::{Library, Symbol};
use std::ffi::CString;

/// Dynamically load a compiled C++ shared library and call a function.
/// The C++ function must have signature: `extern "C" int run(int argc, const char** argv)`
pub fn run(lib_path: &str, func_name: &str, args: &[String]) -> Result<()> {
    let c_args: Vec<CString> = args.iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect();
    let c_ptrs: Vec<*const i8> = c_args.iter().map(|s| s.as_ptr()).collect();

    unsafe {
        let lib = Library::new(lib_path)?;
        let func: Symbol<unsafe extern "C" fn(i32, *const *const i8) -> i32> =
            lib.get(func_name.as_bytes())?;
        let ret = func(c_ptrs.len() as i32, c_ptrs.as_ptr());
        if ret != 0 {
            bail!("C++ function returned {}", ret);
        }
    }
    Ok(())
}
