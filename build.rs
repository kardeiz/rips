fn main() {

    let pkg = pkg_config::probe_library("vips").expect("Could not find libvips");

    let out_path = std::path::Path::new("src/ffi.rs");

    if out_path.exists() { return; }

    let mut builder = bindgen::Builder::default();

    for path in pkg.include_paths.iter()
        .filter_map(|p| p.to_str()) {
        
        builder = builder.clang_arg("-I");
        builder = builder.clang_arg(path);
    }

    let bindings = builder
        .derive_debug(true)
        .impl_debug(true)
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .header_contents("wrapper.h", "#include \"vips/vips.h\"")
        .generate()
        .unwrap();

    bindings.write_to_file(out_path).unwrap();

}
