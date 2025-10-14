use crate::api;
use tokio;

#[tauri::command]
pub async fn download_installer(state: tauri::State<'_, api::AppState>) -> Result<(), String> {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    Ok(())
}

#[tauri::command]
pub async fn run_installation() -> Result<(), String> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(())
}
