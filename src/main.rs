use std::{path::Path, cmp::Reverse};
use bpaf::Bpaf;
use console::Term;

mod menu;
mod project;
mod search;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
/// Remove unwanted dependencies and build artifacts from local projects
pub struct Options {
    /// Verbose output
    #[bpaf(short, long)]
    pub verbose: bool,

    /// Include projects not tracked by supported version control systems
    #[bpaf(long)]
    pub no_vcs: bool,

    /// Don't sort indexed projects
    #[bpaf(short, long)]
    pub unsorted: bool,

    /// Hide projects with zero possible disk savings
    #[bpaf(short, long)]
    pub skip_empty: bool,

    /// Don't bring up project menu (for testing purposes only)
    #[bpaf(long)]
    pub dry_run: bool,

    #[bpaf(positional("DIR"))]
    /// Directory to search for projects [default: .]
    pub dir: Option<String>,
}

fn main() {
    const MAX_SEARCH_DEPTH: u32 = 10; // only applies if --no-vcs flag is specified

    let args = options().run();

    let path_str = &args.dir.unwrap_or(".".to_string());
    let path = Path::new(path_str);
    if !path.exists() {
        println!("Path '{}' does not exist.", path.display());
        return;
    }
    if path.is_file() {
        println!("'{}' is a file, please specify a directory.", path.display());
        return;
    }

    if !args.dry_run {
        let term_height = Term::stdout().size().0 as usize;
        let top_pad = "\n".repeat(term_height / 2 - 3);
        let bottom_pad = "\n".repeat(term_height / 2 - 5);
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
    }

    let mut projects =
        if args.no_vcs {
            search::find_projects(path,MAX_SEARCH_DEPTH)
        } else {
            search::find_git_projects(path)
        };

    if args.skip_empty {
        projects.retain(|p| p.rm_size > 0);
    }
    if projects.is_empty() {
        println!("No projects found.");
        return;
    }

    if !args.unsorted {
        projects.sort_unstable_by_key(|p| Reverse(p.rm_size));
        projects.sort_by_key(|p| p.project_type);
    }
    
    if !args.dry_run {
        menu::project_menu(projects, args.verbose);
    }
}
