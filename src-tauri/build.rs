use std::{env, path::Path};

fn main() {
    let lib_path = Path::new(".\\lib").to_path_buf();
    let include_path = Path::new(".\\inc").to_path_buf();

    println!("cargo:rustc-link-search={}", lib_path.to_string_lossy());
    println!("cargo:rustc-link-lib=TKernel");
    println!("cargo:rustc-link-lib=TKMath");
    println!("cargo:rustc-link-lib=TKBRep");
    println!("cargo:rustc-link-lib=TKXSBase");
    println!("cargo:rustc-link-lib=TKSTEP");
    println!("cargo:rustc-link-lib=TKSTL");
    println!("cargo:rustc-link-lib=TKMesh");

    cxx_build::bridge("src/main.rs")
        .file("src/OCCTWrapper.cpp")
        .include(include_path)
        .include("src")
        .flag_if_supported("-std=c++14")
        .compile("occtwrapper");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.cpp");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.hpp");

    println!(
        "cargo:rustc-env=PATH={};{}",
        Path::new("../dlls").to_string_lossy(),
        env::var("PATH").unwrap(),
    );

    tauri_build::build()
}
