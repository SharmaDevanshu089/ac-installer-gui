use crate::api;
use tokio;

const DEBUG: bool = true;
pub fn get_directories(type_of: &str) -> PathBuf {
    let app =
        directories::ProjectDirs::from("io", "sharmadevanshu089", "autocrate-install").expect(SRS);
    let temp_path = app.cache_dir();
    let error_fix = format!("get directories was called with {}", type_of);
    let mut out = PathBuf::new();
    match type_of {
        "tmp" => out = temp_path.to_owned(),
        _ => log_error(&error_fix),
    }
    if DEBUG {
        let tmp = out.clone();
        println!("GETDIRECTORIES RESPONDED : {}", tmp.to_string_lossy())
    }
    return out;
}
