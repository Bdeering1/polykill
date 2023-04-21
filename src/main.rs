use std::path::Path;
use clap::Parser;
use project::print_projects;

mod args;
mod project;
mod search;
mod project_type;

fn main() {
    let args = args::PolykillArgs::parse();
    
    println!("Searching for projects...");
    let projects = search::find_git_projects(Path::new(args.dir.as_str()));
    print_projects(&projects);
}
