use std::{env, fs, path::Path};

#[cfg(windows)]
use dunce::canonicalize;

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

    let mut wxs = r#"<?xml version="1.0" encoding="utf-8"?>
    <Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
      <Fragment>
        <DirectoryRef Id="INSTALLDIR">
            <Component Id="occt" Guid="81db30f0-b8c4-45dd-b5d0-12e5d3cd0a8c">
    "#
    .to_string();
    let mut key_path_set = false;
    for entry in Path::new("../dlls").read_dir().unwrap() {
        let f = entry.unwrap();
        let path = {
            if cfg!(windows) {
                canonicalize(f.path()).unwrap()
            } else {
                f.path().canonicalize().unwrap()
            }
        };
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext == "dll" {
            let name = path.file_name().unwrap().to_string_lossy();
            let name = name
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>();
            let key_path = {
                if !key_path_set {
                    key_path_set = true;
                    "yes"
                } else {
                    "no"
                }
            };
            wxs.push_str(&format!(
                "<File Id=\"{name}\" Source=\"{}\" KeyPath=\"{key_path}\" Checksum=\"yes\"/>",
                path.to_string_lossy()
            ));
        }
    }
    wxs.push_str("</Component></DirectoryRef></Fragment></Wix>");
    let wix_dir = Path::new("wix");
    if !wix_dir.is_dir() {
        fs::create_dir(wix_dir).unwrap();
    }
    fs::write(wix_dir.join("dll.wxs"), wxs).unwrap();

    tauri_build::build()
}
