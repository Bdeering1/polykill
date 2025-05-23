use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::{metadata, read_dir, remove_dir_all, remove_file, ReadDir};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub rm_paths: Vec<PathBuf>,
    pub rm_size: u64,
    pub rm_size_str: String,
    pub last_modified: Option<u64>,
}

impl Project {
    pub fn new(path: PathBuf, project_type: ProjectType, rm_paths: Vec<PathBuf>) -> Project {
        let rm_size = get_rm_size(&rm_paths);
        let rm_size_str = bytes_to_string(rm_size);
        let last_modified = get_time_since_last_mod(&path);
        Project {
            path,
            project_type,
            rm_paths,
            rm_size,
            rm_size_str,
            last_modified,
        }
    }

    pub fn cargo(path: PathBuf) -> Project {
        let rm_paths = vec![path.join(PathBuf::from("target"))];
        Project::new(path, ProjectType::Cargo, rm_paths)
    }

    pub fn composer(path: PathBuf) -> Project {
        let rm_paths = vec![path.join(PathBuf::from("vendor"))];
        Project::new(path, ProjectType::Composer, rm_paths)
    }

    pub fn dotnet(path: PathBuf) -> Project {
        let rm_paths = vec![
            path.join(PathBuf::from("bin")),
            path.join(PathBuf::from("obj")),
        ];
        Project::new(path, ProjectType::Dotnet, rm_paths)
    }

    pub fn golang(path: PathBuf) -> Project {
        let dir_name = PathBuf::from(path.file_name().unwrap());
        let rm_paths = if !cfg!(windows) {
            vec![
                path.join(dir_name.to_owned()),
                path.join(dir_name.with_extension("test")),
            ]
        } else {
            vec![
                path.join(dir_name.to_owned().with_extension("exe")),
                path.join(dir_name.with_extension("test.exe")),
            ]
        };
        Project::new(path, ProjectType::Golang, rm_paths)
    }

    pub fn gradle(path: PathBuf) -> Project {
        let rm_paths = vec![path.join(PathBuf::from("build"))];
        Project::new(path, ProjectType::Gradle, rm_paths)
    }

    pub fn misc(path: PathBuf, rm_paths: Vec<PathBuf>) -> Project {
        Project::new(path, ProjectType::Misc, rm_paths)
    }

    pub fn mix(path: PathBuf) -> Project {
        let rm_paths = vec![
            path.join(PathBuf::from("_build")),
            path.join(PathBuf::from("deps")),
        ];
        Project::new(path, ProjectType::Mix, rm_paths)
    }

    pub fn node(path: PathBuf) -> Project {
        let rm_paths = vec![path.join(PathBuf::from("node_modules"))];
        Project::new(path, ProjectType::Node, rm_paths)
    }

    pub fn delete(&mut self) -> Option<String> {
        let mut message = String::from("");
        for path in &self.rm_paths {
            let res = if path.is_dir() {
                remove_dir_all(path)
            } else {
                remove_file(path)
            };

            match res {
                Ok(_) => message += format!("Removed {:?}\n", path).as_str(),
                Err(e) => message += format!("Unable to remove {:?}: {}\n", path, e).as_str(),
            }
        }
        self.rm_size_str = bytes_to_string(self.rm_size);
        self.last_modified = get_time_since_last_mod(&self.path);

        message.pop();
        if message.is_empty() {
            None
        } else {
            Some(message)
        }
    }

    pub fn path_string(&self) -> String {
        self.path.display().to_string()
    }

    pub fn trunc_path_string(&self, truncate_to: usize) -> String {
        let (path, truncated) = truncate_path(&self.path, truncate_to);
        if truncated { return format!("../{}", path.display()) }

        path.display().to_string()
    }

    pub fn type_string(&self) -> String {
        if self.project_type == ProjectType::Misc {
            return format!("Misc ({})", self.get_rm_path_str())
        }
        format!("{:?}", self.project_type)
    }

    fn get_rm_path_str(&self) -> &str {
        self.rm_paths[0].file_name().unwrap().to_str().unwrap()
    }
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub enum ProjectType {
    Cargo,
    Composer,
    Dotnet,
    Golang,
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

fn truncate_path(path: &PathBuf, n: usize) -> (PathBuf, bool) {
    let components: Vec<&OsStr> = path.iter().collect();
    if n >= components.len() { return (path.to_owned(), false) }

    (components.iter().skip(components.len() - n).collect(), true)
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

fn get_rm_size(rm_paths: &Vec<PathBuf>) -> u64 {
    let mut size = 0;
    for path in rm_paths {
        let path_exists = path.try_exists();
        if path_exists.is_err() { continue; }

        let entry_size = compute_size(path);
        if entry_size.is_err() { continue; }

        size += entry_size.unwrap();
    }
    size
}

fn compute_size(path: &PathBuf) -> io::Result<u64> {
    if !path.is_dir() {
        return Ok(path.metadata()?.len());
    }

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
