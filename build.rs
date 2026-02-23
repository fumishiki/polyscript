fn main() {
    println!("cargo:rerun-if-changed=include/polyscript_ffi.h");

    let bindings = bindgen::Builder::default()
        .header("include/polyscript_ffi.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen: unable to generate polyscript_ffi.rs");

    // Rust 2024: extern blocks must be `unsafe extern`
    let content = bindings.to_string().replace("extern \"C\" {", "unsafe extern \"C\" {");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(out.join("polyscript_ffi.rs"), content)
        .expect("bindgen: unable to write polyscript_ffi.rs");
}
