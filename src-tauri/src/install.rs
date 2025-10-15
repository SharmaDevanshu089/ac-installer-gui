use crate::api;
use env;
use reqwest;
use std::path::PathBuf;
use tauri::State;
use tokio;

const DEBUG: bool = true;
pub fn get_directories(type_of: &str) -> PathBuf {
    let app =
        directories::ProjectDirs::from("io", "sharmadevanshu089", "autocrate-install").unwrap();
    let temp_path = app.cache_dir();
    let error_fix = format!("get directories was called with {}", type_of);
    let mut out = PathBuf::new();
    match type_of {
        "tmp" => out = temp_path.to_owned(),
        //jugad to be fixed
        _ => {
            let a = 1;
        }
    }
    if DEBUG {
        let tmp = out.clone();
        println!("GETDIRECTORIES RESPONDED : {}", tmp.to_string_lossy())
    }
    return out;
}
#[tauri::command]
pub async fn download_binary(state: tauri::State<'_, api::AppState>) -> Result<(), String> {
    let appdata =
        env::var("APPDATA").expect("Unable to get a envirment variable, i hope yours is supported");
    let mut appdata_path = PathBuf::from(appdata.clone());
    appdata_path.push(".autocrate");
    let autocrate_path = appdata_path.clone();
    appdata_path.push("autocrate.exe");
    if DEBUG {
        println!("{}", appdata_path.to_string_lossy());
    }
    let url = "";
    let response = reqwest::get(url);
    Ok("Sucess Download".to_string())
}
