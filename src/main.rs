use std::{path::PathBuf, cmp::Reverse};
use clap::Parser;
use console::Term;

mod auto;
mod menu;
mod project;
mod search;
mod service;

const ANSI_HIDE_CURSOR: &str = "\x1b[?25l";
const ANSI_SHOW_CURSOR: &str = "\x1b[?25h";

#[derive(Debug, Parser)]
#[clap(author, version, verbatim_doc_comment)]
/// Remove unwanted dependencies and build artifacts from local projects
pub struct PolykillArgs {
    #[clap(default_value = ".")]
    /// List of directories to search for projects
    pub dirs: Vec<String>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Automatically clean up older project artifacts (no menu)
    #[arg(short, long)]
    pub auto: bool,

    /// Minimum threshold for automatic cleanup (days since last modified)
    #[arg(short, long, default_value_t = service::DEFAULT_CLEANUP_THRESHOLD)]
    pub threshold: u64,

    /// Register system service to run polykill automatically
    #[arg(long)]
    pub register: bool,

    /// Remove registered system service
    #[arg(long)]
    pub unregister: bool,

    /// Interval (in days) to run system service on
    #[arg(short, long, default_value_t = service::DEFAULT_INTERVAL)]
    pub interval: u64,

    /// Check if a polykill service exists, print info if found
    #[arg(long)]
    pub status: bool,

    /// Print polykill service logs, if any
    #[arg(long)]
    pub logs: bool,

    /// Hide projects with zero possible disk savings
    #[arg(short, long)]
    pub skip_empty: bool,

    /// Don't sort indexed projects
    #[arg(short, long)]
    pub unsorted: bool,

    /// Include projects not tracked by supported version control systems
    #[arg(long)]
    pub no_vcs: bool,

    /// Don't bring up project menu (for testing purposes only)
    #[arg(long)]
    pub dry_run: bool,
}

fn main() {
    const MAX_SEARCH_DEPTH: u32 = 10; // only applies if --no-vcs flag is specified

    let args = PolykillArgs::parse();

    if args.logs {
        service::logs();
        return;
    }

    let mut search_paths = Vec::with_capacity(args.dirs.len());
    for path_str in args.dirs {
        let path = PathBuf::from(path_str);
        if !path.exists() {
            println!("Path '{}' does not exist.", path.display());
            return;
        }
        if !path.is_dir() {
            println!("'{}' is a file, please specify a directory.", path.display());
            return;
        }
        search_paths.push(path);
    }

    if args.register {
        service::register(search_paths, args.interval, args.threshold);
        return;
    }
    if args.unregister {
        service::unregister();
        return;
    }
    if args.status{
        service::status();
        return;
    }

    if !args.dry_run && !args.auto {
        let term_height = Term::stdout().size().0 as usize;
        let top_pad = "\n".repeat(term_height / 2 - 6);
        let bottom_pad = "\n".repeat(term_height / 2 - 3);
        print!("{}{}
        ██████   ██████  ██   ██    ██ ██   ██ ██ ██      ██
        ██   ██ ██    ██ ██    ██  ██  ██  ██  ██ ██      ██ 
        ██████  ██    ██ ██     ████   █████   ██ ██      ██        
        ██      ██    ██ ██      ██    ██  ██  ██ ██      ██   
        ██       ██████  ███████ ██    ██   ██ ██ ███████ ███████ 
        v{}
        
        searching for projects...{}",
        ANSI_HIDE_CURSOR, top_pad, env!("CARGO_PKG_VERSION"), bottom_pad
        );
    }

    let mut projects = Vec::<project::Project>::new();
    for path in search_paths {
        if args.no_vcs {
            projects.append(&mut search::find_projects(&path, MAX_SEARCH_DEPTH));
        } else {
            projects.append(&mut search::find_git_projects(&path));
        }
    }

    if args.auto {
        auto::run(projects, args.threshold);
        return;
    }

    if args.skip_empty {
        projects.retain(|p| p.rm_size > 0);
    }
    if projects.is_empty() {
        println!("{}{}No projects found.", menu::ANSI_CLEAR_SCREEN, ANSI_SHOW_CURSOR);
        return;
    }
    
    if args.dry_run { return; }

    if !args.unsorted {
        projects.sort_unstable_by_key(|p| Reverse(p.rm_size));
        projects.sort_by_key(|p| p.project_type);
    }

    menu::project_menu(projects, args.verbose);
}
