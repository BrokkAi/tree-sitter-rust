fn main() {
    let src_dir = std::path::Path::new("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);
    c_config
        .define("tree_sitter_rust", "brokk_tree_sitter_rust")
        .define(
            "tree_sitter_rust_external_scanner_create",
            "brokk_tree_sitter_rust_external_scanner_create",
        )
        .define(
            "tree_sitter_rust_external_scanner_destroy",
            "brokk_tree_sitter_rust_external_scanner_destroy",
        )
        .define(
            "tree_sitter_rust_external_scanner_scan",
            "brokk_tree_sitter_rust_external_scanner_scan",
        )
        .define(
            "tree_sitter_rust_external_scanner_serialize",
            "brokk_tree_sitter_rust_external_scanner_serialize",
        )
        .define(
            "tree_sitter_rust_external_scanner_deserialize",
            "brokk_tree_sitter_rust_external_scanner_deserialize",
        );

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let scanner_path = src_dir.join("scanner.c");
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    c_config.compile("brokk-tree-sitter-rust");

    println!("cargo:rustc-check-cfg=cfg(with_highlights_query)");
    if std::path::Path::new("queries/highlights.scm").exists() {
        println!("cargo:rustc-cfg=with_highlights_query");
    }
    println!("cargo:rustc-check-cfg=cfg(with_injections_query)");
    if std::path::Path::new("queries/injections.scm").exists() {
        println!("cargo:rustc-cfg=with_injections_query");
    }
    println!("cargo:rustc-check-cfg=cfg(with_locals_query)");
    if std::path::Path::new("queries/locals.scm").exists() {
        println!("cargo:rustc-cfg=with_locals_query");
    }
    println!("cargo:rustc-check-cfg=cfg(with_tags_query)");
    if std::path::Path::new("queries/tags.scm").exists() {
        println!("cargo:rustc-cfg=with_tags_query");
    }
}
