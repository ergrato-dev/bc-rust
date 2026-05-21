fn main() {
    // Genera el header C automáticamente con cbindgen.
    // Solo se ejecuta si está disponible cbindgen.
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::from_file("cbindgen.toml")
        .unwrap_or_default();
    let _ = cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .map(|bindings| bindings.write_to_file("capstone_d.h"));
}
