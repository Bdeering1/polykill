use std::path::Path;
use clap::Parser;
use console::Term;
use menu::project_menu;

mod menu;
mod project;
mod search;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct PolykillArgs {
    #[clap(default_value_t = String::from("."))]
    /// The directory to search for projects
    pub dir: String,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Include projects not tracked by git
    #[arg(short, long)]
    pub no_git: bool,

    /// Hide projects with zero possible disk savings
    #[arg(long)]
    pub hide_empty: bool,

}

fn main() {
    const MAX_SEARCH_DEPTH: u32 = 10; // only applies if --no-git flag is specified

    let args = PolykillArgs::parse();
    let path = Path::new(args.dir.as_str());
    if !path.exists() {
        println!("Directory '{}' does not exist.", path.display());
        return;
    }
    if path.is_file() {
        println!("'{}' is a file, please specify a directory.", path.display());
        return;
    }

    let term_height = Term::stdout().size().0 as usize;
    let top_pad = format!("{}", "\n".repeat(term_height / 2 - 3));
    let bottom_pad = format!("{}", "\n".repeat(term_height / 2 - 5));
    println!("
    {}
    ██████   ██████  ██   ██    ██ ██   ██ ██ ██      ██  
    ██   ██ ██    ██ ██    ██  ██  ██  ██  ██ ██      ██ 
    ██████  ██    ██ ██     ████   █████   ██ ██      ██        
    ██      ██    ██ ██      ██    ██  ██  ██ ██      ██   
    ██       ██████  ███████ ██    ██   ██ ██ ███████ ███████ 
    v{}
    
    searching for projects...
    {}
    ", top_pad, env!("CARGO_PKG_VERSION"), bottom_pad
    );

    let mut projects =
        if args.no_git {
            search::find_projects(path,MAX_SEARCH_DEPTH)
        } else {
            search::find_git_projects(path)
        };

    if args.hide_empty {
        projects.retain(|p| p.rm_size > 0);
    }
    if projects.is_empty() {
        println!("No projects found.");
        return;
    }

    project_menu(projects, args.verbose);
}