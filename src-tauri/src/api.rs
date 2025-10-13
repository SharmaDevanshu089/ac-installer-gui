use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const URL: &str = "https://api.github.com/repos/SharmaDevanshu089/AutoCrate/releases/latest";
const DEBUG: bool = false;

pub struct AppState {
    pub download_url: Mutex<Option<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssetInfo {
    pub name: String,
    pub content_type: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReturnData {
    pub tag_name: String,
    pub published_at: String,
    pub name: Option<String>,
    pub browser_download_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub published_at: String,
    pub name: Option<String>,
    pub assets: Vec<AssetInfo>,
}

#[tauri::command]
pub async fn get_release_data() -> Result<ReturnData, String> {
    let client = Client::new();
    let github_said = client
        .get(URL)
        .header("User-Agent", "AutoCrate Installer")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<ReleaseInfo>()
        .await
        .map_err(|e| e.to_string())?;
    if let Some(selected_installer) = github_said
        .assets
        .iter()
        .find(|asset| asset.content_type == "application/x-msdownload")
    {
        let data_for_frontend = ReturnData {
            name: github_said.name.clone(),
            tag_name: github_said.tag_name.clone(),
            published_at: github_said.published_at.clone(),
            browser_download_url: selected_installer.browser_download_url.clone(),
        };
        if DEBUG {
            println!("{}", data_for_frontend.browser_download_url);
        }
        Ok(data_for_frontend)
    } else {
        panic!();
    }
}
