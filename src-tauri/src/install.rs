use crate::api;
use env;
use reqwest;
use std::fs::{write, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
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
pub async fn download_binary(state: tauri::State<'_, api::AppState>) -> Result<String, String> {
    let appdata =
        env::var("APPDATA").expect("Unable to get a envirment variable, i hope yours is supported");
    let mut appdata_path = PathBuf::from(appdata.clone());
    appdata_path.push(".autocrate");
    std::fs::create_dir_all(appdata_path.clone());
    let autocrate_path = appdata_path.clone();
    appdata_path.push("autocrate.exe");
    if DEBUG {
        println!("{}", appdata_path.to_string_lossy());
    }
    let url = state.download_url.lock().unwrap().clone();
    let uri = url.unwrap();
    let response = reqwest::get(uri).await.expect("Error 1");
    let content = response.bytes().await.expect("Error 2");
    let mut file = File::create(appdata_path).expect("Error");
    file.write_all(&content).expect("Error 4");
    Ok("Sucess Download".to_string())
}
fn add_to_path() {
    let appdata =
        env::var("APPDATA").expect("Unable to get a envirment variable, i hope yours is supported");
    let mut appdata_path = PathBuf::from(appdata.clone());
    appdata_path.push(".autocrate");
    let app_path = appdata_path.clone();
    println!("Configuring System for install.");
    let path_to_change = match env::var("Path") {
        Ok(p) => p,
        Err(p) => {}
    };
    let folder_path = app_path.to_string_lossy();
    if DEBUG {
        let tmp = folder_path.clone();
        println!("Folder_Path is {}", tmp);
    }
    let new_path = format!("{};{}", path_to_change, folder_path);
    if DEBUG {
        let tmp = new_path.clone();
        println!("New Path : {}", tmp);
    }
    let ps_script = format!(
        "[Environment]::SetEnvironmentVariable('PATH','{}','User')",
        new_path
    );
    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(ps_script)
        .status()
        .expect("Unable to update PATH via PowerShell");
    //let status = Command::new("setx").arg("PATH").arg(new_path).status().expect("Unable to Set new Path");
    if !status.success() {
        println!("There has been a problem with your system.");
    }
}
