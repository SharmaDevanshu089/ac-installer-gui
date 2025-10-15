#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO: Add Internet Check
// TODO: Add Loading when Internet Connecting in fetch function
// TODO: Add Caching for the API Connection.
// TODO: Add a Browser Donwload option
// TODO: Add edge case handling where there are no assets
// TODO: Add edge case for handling multiple executable in release
// TODO: Deal with panic in api or it is given above
// TODO: Add ability to change the installation directory
// TODO: Use Streaming instead of downloading to ram
// TODO: Just Implement a Basic Download Verification , like just compare the result return by rust or create a error

mod install;

mod api;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(api::AppState {
            download_url: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            api::get_release_data,
            install::download_binary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
