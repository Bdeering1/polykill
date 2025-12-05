use chrono::Local;

use crate::project::Project;

pub fn run(mut projects: Vec<Project>, threshold: u64) {
    log("Running polykill system service.");

    let mut cleaned = 0;
    for p in &mut projects {
        if p.rm_size == 0 || p.last_modified == None || p.last_modified.unwrap() < threshold { continue; }
        cleaned += 1;

        let message = p.delete();
        if let Some(msg) = message {
            let msg = msg.replace("\n", ",");
            log(&msg);
        }
    }

    let summary = format!("Found {} projects, cleaned up {}.", projects.len(), cleaned);
    log(&summary);
}

fn log(msg: &str) {
    let local_time = Local::now().to_rfc3339();
    println!("[{}] {}", local_time, msg);
}
