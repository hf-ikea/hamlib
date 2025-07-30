use autotools;
use autotools::Config;
use std::env;
use std::path::PathBuf;

pub fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping Hamlib native build for docs.rs.");
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

    let dst = Config::new("src/native/Hamlib")
        .reconf("-i")
        .cflag("-Wall")
        .build();

    // Simply link the library without using pkg-config
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=hamlib");
    let bindings = bindgen::Builder::default()
        .header("src/native/wrapper.h")
        .detect_include_paths(true)
        .clang_arg(format!("-I{}", dst.join("include").join("hamlib").display()))
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .clang_macro_fallback()
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}
