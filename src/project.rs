use std::fmt::{Display, Formatter};
use std::fs::{ReadDir, metadata, read_dir, remove_dir_all};
use std::path::PathBuf;
use std::io;
use std::time::{SystemTime, Duration};
use bytesize::ByteSize;

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub rm_dirs: Vec<PathBuf>,
    pub rm_size: u64,
    pub rm_size_str: String,
    pub last_modified: String,
}

impl Project {
    pub fn new(path: PathBuf, project_type: ProjectType, rm_dirs: Vec<PathBuf>) -> Project {
        let rm_size = get_rm_size(&rm_dirs);
        let rm_size_str = ByteSize::b(rm_size as u64).to_string();
        let last_modified = get_time_since_last_mod(&path);
        Project { path, project_type, rm_dirs, rm_size, rm_size_str, last_modified }
    }

    pub fn node(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("node_modules"))];
        Project::new(path, ProjectType::Node, rm_dirs)
    }

    pub fn cargo(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("target"))];
        Project::new(path, ProjectType::Cargo, rm_dirs)
    }

    pub fn mix(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("_build")), path.join(PathBuf::from("deps"))];
        Project::new(path, ProjectType::Mix, rm_dirs)
    }

    pub fn dotnet(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("bin")), path.join(PathBuf::from("obj"))];
        Project::new(path, ProjectType::Dotnet, rm_dirs)
    }
}

#[derive(Debug)]
pub enum ProjectType {
    Node,
    Cargo,
    Dotnet,
    Mix
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_time_since_last_mod(path: &PathBuf) -> String {
    const SECONDS_PER_DAY: u64 = 86400;
    let meta = metadata(path);

    if meta.is_err() { return String::from("Unknown"); }
    let meta = meta.unwrap();
    let last_mod = meta.modified();

    if last_mod.is_err() { return String::from("Unknown"); }
    let last_mod = last_mod.unwrap();
    let time_since = SystemTime::now().duration_since(last_mod);

    if time_since.is_err() { return String::from("Unknown"); }
    let time_since_days = time_since.unwrap().as_secs() / Duration::from_secs(SECONDS_PER_DAY).as_secs();
    format!("{} days", time_since_days)
}

fn get_rm_size(rm_dirs: &Vec<PathBuf>) -> u64 {
    let mut size = 0;
    for dir in rm_dirs {
        let path_exists = dir.try_exists();
        if path_exists.is_err() { continue; }

        let dir_size = dir_size(dir);
        if dir_size.is_err() { continue; }

        size += dir_size.unwrap();
    }
    size
}

fn dir_size(path: &PathBuf) -> io::Result<u64> {
    fn dir_size(mut dir: ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    dir_size(read_dir(path)?)
}

pub fn delete(dirs: &Vec<PathBuf>) {
    for dir in dirs {
        match remove_dir_all(dir) {
            Ok(_) => println!("Removed {:?}", dir),
            Err(e) => println!("Unable to remove {:?}: {}", dir, e)
        }
    }
}