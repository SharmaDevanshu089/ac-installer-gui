use reqwest::Client;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.github.com/repos/SharmaDevanshu089/AutoCrate/releases/latest";

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
pub async fn get_release_data() -> Result<ReleaseInfo, String> {
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
    Ok(github_said)
}
