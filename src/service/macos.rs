use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, remove_file, write};
use std::io::{stdin, stdout, Write};
use std::path::{PathBuf};
use std::process::Command;

use crate::service::ServiceManager;

const SECONDS_PER_DAY: u64 = 86400;
const SERVICE_NAME: &str = "polykill";
const SERVICE_LABEL: &str = "io.github.bdeering1.polykill";

const LAUNCHD_TEMPLATE: &str = include_str!("../../templates/macos/launchd.plist.template");

pub struct MacOSService {}

impl ServiceManager for MacOSService {
    fn install(search_paths: Vec<PathBuf>, interval: u64, threshold: u64) -> Result<PathBuf, Box<dyn Error>> {
        let mut paths_xml = String::from("");
        for p in search_paths  {
            let path_str = p.to_str();
            if path_str.is_none() {
                return Err(format!("Path contains invalid UTF-8: {:?}", p).into());
            }
            paths_xml.push_str(&format!("        <string>{}</string>\n", xml_escape(path_str.unwrap())));
        }
        paths_xml = paths_xml.trim().to_owned();

        let status_output = Command::new("launchctl")
            .args(["list", SERVICE_LABEL])
            .output()?;
        if status_output.status.success() {
            let mut input = String::new();

            loop {
                print!("A service is already registered. Remove and re-register? [y/n]: ");
                stdout().flush().unwrap();

                String::clear(&mut input);
                stdin().read_line(&mut input).expect("Error: unable to read user input");
                input = input.trim().to_lowercase().to_owned();

                match input.as_str() {
                    "y" => { Self::uninstall()?; break }, 
                    "n" => return Err("Canceled by user".into()),
                    _ => continue,
                }
            }
        }

        let seconds_interval = (interval * SECONDS_PER_DAY).to_string();
        let cleanup_threshold = threshold.to_string();

        let binary_path = std::env::current_exe()?;

        let log_dir = get_log_dir()?;
        let log_path = format!("{}/{}.log", log_dir.to_str().unwrap(), SERVICE_NAME);
        create_dir_all(&log_dir)?;

        let mut template = LAUNCHD_TEMPLATE.to_owned();
        let replacements = HashMap::from([
            ("{{SERVICE_NAME}}", SERVICE_NAME),
            ("{{SERVICE_LABEL}}", SERVICE_LABEL),
            ("{{BINARY_PATH}}", binary_path.to_str().unwrap()),
            ("{{LOG_PATH}}", &log_path),
            ("{{CLEANUP_THRESHOLD}}", &cleanup_threshold),
            ("{{INTERVAL_SECONDS}}", &seconds_interval),
            ("{{SEARCH_PATHS}}", &paths_xml),
        ]);

        for (placeholder, value) in replacements {
            template = template.replace(placeholder, value);
        }

        let launch_agents_dir = get_home_dir()?.join("Library/LaunchAgents");
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

        Ok(plist_path)
    }

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

    fn status() -> Option<String> {
        let output = Command::new("launchctl")
            .args(["list", SERVICE_LABEL])
            .output();
        if let Err(_) = output { return None; }

        let output = output.unwrap();
        if !output.status.success() { return None; }

        Some(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn logs() -> Result<String, Box<dyn std::error::Error>> {
        let log_path = format!("{}/{}.log", get_log_dir()?.to_str().unwrap(), SERVICE_NAME);
        let output = Command::new("tail")
            .args(["--lines", "15", &log_path])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(stdout)
    }
}

fn get_log_dir() -> Result<PathBuf, Box<dyn Error>> {
    let log_dir = get_home_dir()?.join("Library/Logs/polykill");

    Ok(log_dir)
}

fn get_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    dirs::home_dir().ok_or("Could not find home directory".into())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
