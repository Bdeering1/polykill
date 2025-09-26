use std::io::{stdin, stdout, Write};

use crate::project::Project;

pub const DEFAULT_CLEANUP_THRESHOLD: u64 = 60;
pub const DEFAULT_INTERVAL: u64 = 7;

pub fn run(projects: Vec<Project>, threshold: u64) {
    for mut p in projects {
        if p.rm_size == 0 || p.last_modified == None || p.last_modified.unwrap() < threshold { continue; }

        let message = p.delete();
        if let Some(msg) = message {
            println!("{}", msg);
        }
    }
}

pub fn register(mut threshold: u64) {
    let mut interval = DEFAULT_INTERVAL;
    let mut input = String::new();

    loop {
        print!("How often should polykill be run? [default: {}]: ", DEFAULT_INTERVAL);
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

    println!("Polykill will register a system service to run every {} days and remove artifacts from projects last modified >= {} days ago.", interval, threshold);
    loop {
        print!("Is this okay? [y/n]: ");
        stdout().flush().unwrap();

        String::clear(&mut input);
        stdin().read_line(&mut input).expect("Error: unable to read user input");
        input = input.trim().to_lowercase().to_owned();

        match input.as_str() {
            "y" => break, 
            "n" => return,
            _ => continue,
        }
    }

    #[cfg(target_os = "macos")]
    println!("Not implemented yet.");

    #[cfg(target_os = "linux")]
    println!("Not implemented yet.");

    #[cfg(target_os = "windows")]
    println!("Not implemented yet.");
}
