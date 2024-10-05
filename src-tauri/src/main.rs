// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use tauri::command;

const DEFAULT_CHORD_ERROR: f64 = 0.005;
const DEFAULT_ANGLE_RES: f64 = 1.;

#[cxx::bridge(namespace = "OcctWrapper")]
mod ffi {
    unsafe extern "C++" {
        include!("step2stl/src/OCCTWrapper.hpp");

        fn convert_step_to_stl(
            step_file_path: String,
            stl_file_path: String,
            chord_error: f64,
            angle_res: f64,
            binary: bool,
        ) -> bool;
    }
}

#[command]
fn convert(path: String, chord_error: f64, angle_res: f64, binary: bool) -> Result<String, String> {
    let path = PathBuf::from(path);
    if !path.is_file() {
        return Err("File not found".to_string());
    }
    let stl_path = get_stl_path(&path);
    let res = ffi::convert_step_to_stl(
        path.to_string_lossy().into_owned(),
        stl_path.to_string_lossy().into_owned(),
        chord_error,
        angle_res,
        binary,
    );
    if !res {
        return Err("Could not convert file to STL".to_string());
    }
    Ok(stl_path.to_string_lossy().into_owned())
}

fn get_stl_path(step_path: impl AsRef<Path>) -> PathBuf {
    let step_path = step_path.as_ref();
    let mut stl_path = step_path.to_path_buf().with_extension("stl");
    // If the STL file already exists, append a number to the filename
    let mut i = 1;
    while stl_path.exists() {
        stl_path = step_path
            .with_file_name(format!(
                "{}_{}.stl",
                step_path.file_stem().unwrap().to_string_lossy(),
                i
            ))
            .to_path_buf();
        i += 1;
    }
    stl_path
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![convert])
        .setup(|app| {
            let args: Vec<String> = env::args().collect();
            if args.len() > 1 {
                let path = args[1].clone();
                convert(path, DEFAULT_CHORD_ERROR, DEFAULT_ANGLE_RES, false)?;
                let handle = app.handle();
                handle.exit(0);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
