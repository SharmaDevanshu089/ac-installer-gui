#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO: Add Internet Check
// TODO: Add Loading when Internet Connecting in fetch function
// TODO: Add Caching for the API Connection.
// TODO: Add a Browser Donwload option

mod api;
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![api::get_release_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
