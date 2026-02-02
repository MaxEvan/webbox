#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{generate_app, open_in_finder, launch_app};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            generate_app,
            open_in_finder,
            launch_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
