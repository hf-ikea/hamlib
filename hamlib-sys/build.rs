use std::env;

pub fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping SPIRV-Cross native build for docs.rs.");
        return;
    }

    let hamlib_build = cc::Build::new();

    hamlib_build.compile("hamlib");
    println!("cargo:rustc-link-lib=static=spirv-cross");
}