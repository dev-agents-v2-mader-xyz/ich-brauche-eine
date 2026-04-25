#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error running tauri application");

    #[cfg(target_arch = "wasm32")]
    yew::Renderer::<ui::app::App>::new().render();
}
