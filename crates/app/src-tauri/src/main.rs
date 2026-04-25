// Tauri entrypoint — App agent fills in commands and plugins here.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
