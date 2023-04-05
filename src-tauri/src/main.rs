// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cxx::bridge(namespace = "OcctWrapper")]
mod ffi {
    unsafe extern "C++" {
        include!("step2stl/src/OCCTWrapper.hpp");

        fn convert_step_to_stl(
            step_file_path: String,
            stl_file_path: String,
            chord_error: f64,
            angle_res: f64,
        ) -> bool;
    }
}

fn main() {
    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
