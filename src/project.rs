use std::fmt::{Display, Formatter};
use std::fs::{metadata, read_dir, remove_dir_all, ReadDir};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub rm_dirs: Vec<PathBuf>,
    pub rm_size: u64,
    pub rm_size_str: String,
    pub last_modified: Option<u64>,
}

impl Project {
    pub fn new(path: PathBuf, project_type: ProjectType, rm_dirs: Vec<PathBuf>) -> Project {
        let rm_size = get_rm_size(&rm_dirs);
        let rm_size_str = bytes_to_string(rm_size);
        let last_modified = get_time_since_last_mod(&path);
        Project {
            path,
            project_type,
            rm_dirs,
            rm_size,
            rm_size_str,
            last_modified,
        }
    }

    pub fn cargo(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("target"))];
        Project::new(path, ProjectType::Cargo, rm_dirs)
    }

    pub fn composer(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("vendor"))];
        Project::new(path, ProjectType::Composer, rm_dirs)
    }

    pub fn dotnet(path: PathBuf) -> Project {
        let rm_dirs = vec![
            path.join(PathBuf::from("bin")),
            path.join(PathBuf::from("obj")),
        ];
        Project::new(path, ProjectType::Dotnet, rm_dirs)
    }

    pub fn gradle(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("build"))];
        Project::new(path, ProjectType::Gradle, rm_dirs)
    }

    pub fn misc(path: PathBuf, rm_dirs: Vec<PathBuf>) -> Project {
        Project::new(path, ProjectType::Misc, rm_dirs)
    }

    pub fn mix(path: PathBuf) -> Project {
        let rm_dirs = vec![
            path.join(PathBuf::from("_build")),
            path.join(PathBuf::from("deps")),
        ];
        Project::new(path, ProjectType::Mix, rm_dirs)
    }

    pub fn node(path: PathBuf) -> Project {
        let rm_dirs = vec![path.join(PathBuf::from("node_modules"))];
        Project::new(path, ProjectType::Node, rm_dirs)
    }

    pub fn delete(&mut self) -> Option<String> {
        let mut message = String::from("");
        for dir in &self.rm_dirs {
            match remove_dir_all(dir) {
                Ok(_) => message += format!("Removed {:?}\n", dir).as_str(),
                Err(e) => message += format!("Unable to remove {:?}: {}\n", dir, e).as_str(),
            }
        }
        self.rm_size = get_rm_size(&self.rm_dirs);
        self.rm_size_str = bytes_to_string(self.rm_size);
        self.last_modified = get_time_since_last_mod(&self.path);

        message.pop();
        if message.is_empty() {
            None
        } else {
            Some(message)
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub enum ProjectType {
    Cargo,
    Composer,
    Dotnet,
    Gradle,
    Misc,
    Mix,
    Node,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_time_since_last_mod(path: &PathBuf) -> Option<u64> {
    const SECONDS_PER_DAY: u64 = 86400;
    let meta = metadata(path);

    if meta.is_err() { return None; }
    let meta = meta.unwrap();
    let last_mod = meta.modified();

    if last_mod.is_err() { return None; }
    let last_mod = last_mod.unwrap();
    let time_since = SystemTime::now().duration_since(last_mod);

    if time_since.is_err() { return None; }
    Some(time_since.unwrap().as_secs() / Duration::from_secs(SECONDS_PER_DAY).as_secs())
}

fn get_rm_size(rm_dirs: &Vec<PathBuf>) -> u64 {
    let mut size = 0;
    for dir in rm_dirs {
        let path_exists = dir.try_exists();
        if path_exists.is_err() {
            continue;
        }

        let dir_size = dir_size(dir);
        if dir_size.is_err() {
            continue;
        }

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

fn bytes_to_string(bytes: u64) -> String {
    const KB: u64 = 1000;
    const BASE: f64 = 6.931471806;
    const PREFIXES: &[u8] = "KMGT".as_bytes();

    if bytes < KB {
        format!("{}  B", bytes)
    } else {
        let size = bytes as f64;
        let exponent = match (size.ln() / BASE) as usize {
            e if e == 0 => 1,
            e => e,
        };

        format!(
            "{:.1} {}B",
            (size / KB.pow(exponent as u32) as f64),
            PREFIXES[exponent - 1] as char
        )
    }
}
