
use std::env::current_dir;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use macos::MacOSService as PlatformService;
#[cfg(target_os = "windows")]
use windows::WindowsService as PlatformService;
#[cfg(target_os = "linux")]
use linux::LinuxService as PlatformService;

pub const DEFAULT_CLEANUP_THRESHOLD: u64 = 60;
pub const DEFAULT_INTERVAL: u64 = 7;

pub trait ServiceManager {
    fn install(search_paths:Vec<PathBuf>, interval: u64, threshold: u64) -> Result<PathBuf, Box<dyn Error>>;
    fn uninstall() -> Result<(), Box<dyn Error>>;
    fn status() -> Option<String>;
    fn logs() -> Result<String, Box<dyn Error>>;
}

pub fn register(search_paths: Vec<PathBuf>, mut interval: u64, mut threshold: u64) {
    let mut input = String::new();

    let abs_paths: Vec<PathBuf> = search_paths
        .iter()
        .map(|p| to_absolute_path(p).unwrap())
        .collect();

    println!("Service will search the following directories:");
    for p in &abs_paths {
        println!("{}", p.to_string_lossy());
    }
    println!();

    if interval == DEFAULT_INTERVAL {
        loop {
            print!("How often should polykill be run (days)? [default: {}]: ", DEFAULT_INTERVAL);
            stdout().flush().unwrap();

            String::clear(&mut input);
            stdin().read_line(&mut input).expect("Error: unable to read user input");
            input = input.trim().to_owned();

            if input.len() == 0 { break; }
            if let Ok(num) = input.parse() {
                interval = num;
                break;
            }
        }
    }

    if threshold == DEFAULT_CLEANUP_THRESHOLD {
        loop {
            print!("Run on projects that were last modified more than how many days ago? [default: {}]: ", DEFAULT_CLEANUP_THRESHOLD);
            stdout().flush().unwrap();

            String::clear(&mut input);
            stdin().read_line(&mut input).expect("Error: unable to read user input");
            input = input.trim().to_owned();

            if input.len() == 0 { break; }
            if let Ok(num) = input.parse() {
                threshold = num;
                break;
            }
        }
    }

    let install_path = PlatformService::install(abs_paths, interval, threshold);
    if let Err(e) = &install_path {
        println!("Unable to register system service: {}", e);
        return;
    }

    let install_path = install_path.unwrap();
    println!("Successfully registered system service at {}", install_path.to_str().unwrap());
}

pub fn unregister() {
    if let Err(e) = PlatformService::uninstall() {
        println!("Unable to remove registered service: {}", e);
        return;
    }

    println!("Successfully removed system service.");
}

pub fn status() {
    let res = PlatformService::status();
    if res == None {
        println!("Service not running or not found");
        return;
    }

    println!("Found registered system service.\n\n{}", res.unwrap());
}

pub fn logs() {
    let logs = PlatformService::logs();
    if let Err(e) = &logs {
        println!("Failed to retrieve service logs: {}", e);
    }

    let logs = logs.unwrap();
    if logs.len() == 0 {
        println!("No service logs found.");
        return;
    }

    print!("{}", logs);
}

fn to_absolute_path(path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if !path.is_absolute() {
        let current_dir = current_dir()?;
        let absolute = current_dir.join(path);
        
        return Ok(absolute.canonicalize()?);
    }

    Ok(path.to_path_buf())
}
