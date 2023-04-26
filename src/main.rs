use std::path::Path;
use clap::Parser;
use menu::project_menu;

mod args;
mod menu;
mod project;
mod project_type;
mod search;

fn main() {
    let args = args::PolykillArgs::parse();
    
    println!("Searching for projects...");
    let projects = search::find_git_projects(Path::new(args.dir.as_str()));
    project_menu(&projects);
}
