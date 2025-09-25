use directories;
use reqwest;
use reqwest::blocking::Response;
use std::arch::x86_64::_MM_FROUND_NO_EXC;
use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::copy;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::process::ExitStatus;
use zip::ZipArchive;

const DEBUG: bool = false;
const URL: &str = "https://api.github.com/repos/SharmaDevanshu089/AutoCrate/releases/latest";
const SRS: &str =
    "There has been a Serious error with the program please use a different version. Crashing";
const LOGFILE: &str = "install.log";
const EXECUTIVE_NAME: &str = "USE_INSTALLER_FIRST";
fn copy_to_location(mut old_binary: zip::read::ZipFile<'_, File>) {
    let appdata =
        env::var("APPDATA").expect("Unable to get a envirment variable, i hope yours is supported");
    let mut appdata_path = PathBuf::from(appdata.clone());
    appdata_path.push(".autocrate");
    let app_path = appdata_path.clone();
    appdata_path.push("autocrate.exe");
    if DEBUG {
        let tmp = appdata_path.clone();
        let tmp2 = appdata.clone();
        let tmp3 = app_path.clone();
        println!(
            "{} \n{}\n{}",
            tmp.to_string_lossy(),
            tmp2,
            tmp3.to_string_lossy()
        );
    }
    let mut executable_dir = create_dir_all(
        appdata_path
            .parent()
            .expect("Unable to push the binary into the systen"),
    )
    .expect("Unable to push binary in system");
    let mut executable = fs::File::create(appdata_path).expect("Cannot Create new executable");
    copy(&mut old_binary, &mut executable).expect("Unable to Write into new executable");
    println!("File Sucessfully Installed");
    add_to_path(app_path);
}
fn add_to_path(app_path: PathBuf) {
    println!("Configuring System for install.");
    let path_to_change = match env::var("Path") {
        Ok(p) => p,
        Err(p) => {
            log_error("Unable to Fetch Path");
            log_error("Unable to Fetch Path");
            exit(0);
        }
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
        exit(0);
    }
}
fn unzip() {
    let mut path = get_directories("tmp");
    create_dir_all(path.clone()).expect("Unable to Create Parent Directiory");
    path.push("download.zip");
    if DEBUG {
        let tmp = path.clone();
        println!("The path to Old Zip : {}", tmp.to_string_lossy());
    }
    let file = File::open(path).expect("Unable to open the Files");
    let mut achive_file = ZipArchive::new(file).expect("unable to extract achive ");
    let binary = ZipArchive::by_name(&mut achive_file, EXECUTIVE_NAME)
        .expect("There was Problem extracting old binary");
    copy_to_location(binary);
}
fn check_cargo() {
    let instruction = "cargo";
    let status = Command::new(instruction).arg("--version").status();
    let mut checker;
    match status {
        Ok(p) => {
            checker = p;
        }
        Err(p) => {
            println!("Cargo not found please install it first.");
            exit(0);
        }
    }
    if !checker.success() {
        println!("There has been some error with cargo.");
        exit(0);
    }
    download_zip();
}
fn download_zip() {
    let url = URL;
    let mut path = get_directories("tmp");
    create_dir_all(path.clone()).expect("Unable to Create Parent Directiory");
    path.push("download.zip");
    if DEBUG {
        println!("{}", path.to_string_lossy());
    }
    let mut file = File::create(path.clone()).expect("Unable to Create the donwloaded file");
    let zip = reqwest::blocking::get(url);
    let mut data;
    match zip {
        Ok(zip) => {
            println!("Sucessfully Donwloaded");
            data = zip;
        }
        Err(zip) => {
            println!("There was a problem with download");
            log_error(&zip.to_string());
            exit(0);
        }
    }
    file.write_all(&data.bytes().expect("I think the data is Corrupted"))
        .expect("Could not write ");
    unzip();
}

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

fn log_error(text: &str) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOGFILE)
        .expect("Unable to Create Log File");
    writeln!(&mut log_file, "{}", text).expect("Unable to write a error into log file");
}
pub fn hello() {
    println!("Hello World");
}
async fn request() -> String {
    let Client = Client::new();
    let github_said = Client
        .get(URL)
        .header("User-Agent", "AutoCrate Installer")
        .send()
        .await
        .expect("Error Happens");
    let release = github_said
        .json::<ReleaseInfo>()
        .await
        .expect("Error Occurs");
    return release.tag_name;
}
