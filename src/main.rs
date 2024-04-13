use std::{env::args, path::Path, cmp::Reverse};
use gumdrop::Options;
use console::Term;

mod menu;
mod project;
mod search;

#[derive(Debug, Options)]
/// Remove unwanted dependencies and build artifacts from local projects
pub struct PolykillArgs {
    /// Directory to search for projects [default: .]
    #[options(free)]
    pub dir: Option<String>,

    /// Print help
    #[options()]
    pub help: bool,

    /// Print version
    #[options()]
    pub version: bool,

    /// Verbose output
    #[options()]
    pub verbose: bool,

    /// Include projects not tracked by supported version control systems
    #[options(no_short)]
    pub no_vcs: bool,

    /// Don't sort indexed projects
    #[options()]
    pub unsorted: bool,

    /// Hide projects with zero possible disk savings
    #[options()]
    pub skip_empty: bool,

    /// Don't bring up project menu (for testing purposes only)
    #[options(no_short)]
    pub dry_run: bool,
}

fn print_usage() {
    println!("\nUsage: polykill [OPTIONS] [DIR]");
    println!("\nFor more information try --help.");
}

fn main() {
    const MAX_SEARCH_DEPTH: u32 = 10; // only applies if --no-vcs flag is specified

    let args = args().skip(1).collect::<Vec<_>>();
    let args = PolykillArgs::parse_args_default(&args);
    let args = match args {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            print_usage();
            return;
        }
    };

    if args.help {
        println!("{}", PolykillArgs::usage());
        return;
    }

    let path_str = args.dir.unwrap_or(".".to_string());
    let path = Path::new(&path_str);
    if !path.exists() {
        println!("Path '{}' does not exist.", path.display());
        print_usage();
        return;
    }
    if path.is_file() {
        println!("'{}' is a file, please specify a directory.", path.display());
        print_usage();
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
