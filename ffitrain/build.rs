fn main() {
    let src_dir = std::path::Path::new("bindings");

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir.join("mime.c");
    c_config.file(&parser_path);

    c_config.compile("mime");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    pkg_config::Config::new().probe("zlib").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
