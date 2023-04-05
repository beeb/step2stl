use std::{env, path::Path};

fn main() {
    let home_dir = home::home_dir().unwrap();
    let mut lib_path = home_dir.join(".nix-profile/lib");
    let mut include_path = home_dir.join(".nix-profile/include/opencascade");
    if cfg!(windows) {
        lib_path = Path::new("C:\\OpenCASCADE-7.7.0-vc14-64\\opencascade-7.7.0\\win64\\vc14\\lib")
            .to_path_buf();
        include_path =
            Path::new("C:\\OpenCASCADE-7.7.0-vc14-64\\opencascade-7.7.0\\inc").to_path_buf();
    }

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

    if cfg!(windows) {
        println!(
            "cargo:rustc-env=PATH={};{}",
            Path::new("./dlls").to_string_lossy(),
            env::var("PATH").unwrap(),
        );
    }

    tauri_build::build()
}
