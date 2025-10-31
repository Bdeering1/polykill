use std::collections::HashMap;
use std::env::current_dir;
use std::io::{stdin, stdout, Write};
use std::fs::{create_dir_all, remove_file, write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::project::Project;

pub const DEFAULT_CLEANUP_THRESHOLD: u64 = 60;
pub const DEFAULT_INTERVAL: u64 = 7;

const SECONDS_PER_DAY: u64 = 86400;
const SERVICE_NAME: &str = "polykill";
const SERVICE_LABEL: &str = "io.github.bdeering1.polykill";
const LAUNCHD_TEMPLATE: &str = include_str!("../templates/macos/launchd.plist.template");

pub fn run(projects: Vec<Project>, threshold: u64) {
    for mut p in projects {
        if p.rm_size == 0 || p.last_modified == None || p.last_modified.unwrap() < threshold { continue; }

        let message = p.delete();
        if let Some(msg) = message {
            println!("{}", msg);
        }
    }
}

pub fn register(search_paths: Vec<PathBuf>, mut threshold: u64) {
    let mut interval = DEFAULT_INTERVAL;
    let mut input = String::new();

    let abs_paths: Vec<PathBuf> = search_paths
        .iter()
        .map(|p| to_absolute_path(p).unwrap())
        .collect();

    let mut paths_xml = String::from("");
    for p in abs_paths {
        let path_str = p.to_str();
        if path_str.is_none() {
            println!("Path contains invalid UTF-8: {:?}", p);
            return;
        }
        paths_xml.push_str(&format!("        <string>{}</string>\n", xml_escape(path_str.unwrap())));
    }
    paths_xml = paths_xml.trim().to_owned();

    loop {
        print!("How often should polykill be run? [default: {} days]: ", DEFAULT_INTERVAL);
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

    if threshold == DEFAULT_CLEANUP_THRESHOLD {
        loop {
            print!("Delete artifacts for projects that were last modified more than how many days ago? [default: {}]: ", DEFAULT_CLEANUP_THRESHOLD);
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

    if let Err(e) = install(paths_xml, interval, threshold) {
        println!("Unable to register system service: {}", e);
        return;
    }

    println!("Successfully registered system service!");
}

pub fn unregister() {
    if let Err(e) = uninstall() {
        println!("Unable to remove registered service: {}", e);
        return;
    }

    println!("Successfully removed system service.");
}

pub fn status() {
    let res = service_status();
    if let Err(e) = res {
        println!("Unable to get service status: {}", e);
        return;
    }

    println!("{}", res.unwrap());
}

#[cfg(target_os = "macos")]
fn install(paths_xml: String, interval: u64, threshold: u64) -> Result<(), Box<dyn std::error::Error>> {
    let status_output = Command::new("launchctl")
        .args(["list", SERVICE_LABEL])
        .output()?;
    if status_output.status.success() {
        let mut input = String::new();

        loop {
            print!("Service is already registered. Remove and re-register? [y/n]: ");
            stdout().flush().unwrap();

            String::clear(&mut input);
            stdin().read_line(&mut input).expect("Error: unable to read user input");
            input = input.trim().to_lowercase().to_owned();

            match input.as_str() {
                "y" => { uninstall()?; break }, 
                "n" => return Err("Canceled by user".into()),
                _ => continue,
            }
        }
    }

    let seconds_interval = (interval * SECONDS_PER_DAY).to_string();
    let cleanup_threshold = threshold.to_string();

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let binary_path = std::env::current_exe()?;

    let log_dir = home_dir.join("Library/Logs/polykill");
    create_dir_all(&log_dir)?;

    let mut template = LAUNCHD_TEMPLATE.to_owned();
    let replacements = HashMap::from([
        ("{{SERVICE_NAME}}", SERVICE_NAME),
        ("{{SERVICE_LABEL}}", SERVICE_LABEL),
        ("{{BINARY_PATH}}", binary_path.to_str().unwrap()),
        ("{{LOG_DIR}}", log_dir.to_str().unwrap()),
        ("{{CLEANUP_THRESHOLD}}", &cleanup_threshold),
        ("{{INTERVAL_SECONDS}}", &seconds_interval),
        ("{{SEARCH_PATHS}}", &paths_xml),
    ]);

    for (placeholder, value) in replacements {
        template = template.replace(placeholder, value);
    }

    let launch_agents_dir = home_dir.join("Library/LaunchAgents");
    create_dir_all(&launch_agents_dir)?;

    let plist_path = launch_agents_dir.join(format!("{}.plist", SERVICE_LABEL));
    write(&plist_path, template)?;

    let output = Command::new("launchctl")
        .args(["load", plist_path.to_str().unwrap()])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() || stderr.contains("failed") || stderr.contains("error") {
        return Err(format!("launchctl load failed: {}", stderr).into());
    }

    if !stderr.is_empty() {
        println!("Warning during service load: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    let plist_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(format!("Library/LaunchAgents/{}.plist", SERVICE_LABEL));

    if !plist_path.exists() {
        return Err("Service not found".into());
    }

    let output = Command::new("launchctl")
        .args(["unload", plist_path.to_str().unwrap()])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let already_unloaded = stderr.contains("Could not find specified service")
        || stderr.contains("No such process");

    if !already_unloaded && !(output.status.success() || stderr.contains("failed") || stderr.contains("error")) {
        return Err(format!("launchctl unload failed: {}", stderr).into())
    }

    remove_file(plist_path)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn service_status() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("launchctl")
        .args(["list", SERVICE_LABEL])
        .output()?;

    if !output.status.success() {
        return Ok("Service not running or not found".to_owned());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(target_os = "linux")]
fn install(inteval: u64, threshold: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("Not yet supported on this platform.");

    OK(())
}
#[cfg(target_os = "linux")]
fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    println!("Not yet supported on this platform.");

    Ok(())
}
#[cfg(target_os = "linux")]
fn service_status() -> Result<String, Box<dyn std::error::Error>>  {
    println!("Not yet supported on this platform.");

    Ok(())
}

#[cfg(target_os = "windows")]
fn install(interval: u64, threshold: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("Not yet supported on this platform.");

    Ok(())
}
#[cfg(target_os = "windows")]
fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    println!("Not yet supported on this platform.");

    Ok(())
}
#[cfg(target_os = "windows")]
fn service_status() -> Result<String, Box<dyn std::error::Error>>  {
    println!("Not yet supported on this platform.");

    Ok(())
}

fn to_absolute_path(path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if !path.is_absolute() {
        let current_dir = current_dir()?;
        let absolute = current_dir.join(path);
        
        return Ok(absolute.canonicalize()?);
    }

    Ok(path.to_path_buf())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
