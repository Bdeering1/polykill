use std::path::Path;
use clap::Parser;
use menu::project_menu;

mod menu;
mod project;
mod search;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct PolykillArgs {
    /// The directory to search for projects
    pub dir: String
}

fn main() {
    let args = PolykillArgs::parse();
    
    println!("Searching for projects...");
    let projects = search::find_git_projects(Path::new(args.dir.as_str()));
    project_menu(&projects);
}
