use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::project_types::ProjectType;

pub struct Project {
    pub path: String,
    pub project_type: ProjectType,
    pub rm_dirs: Vec<PathBuf>,
    //pub rm_size: u32,
}

impl Project {
    pub fn new(path: String, project_type: ProjectType, rm_dirs: Vec<PathBuf>) -> Project {
        Project { path, project_type, rm_dirs }
    }

    pub fn node(path: String) -> Project {
        let rm_dirs = vec![PathBuf::from("node_modules")];
        Project::new(path, ProjectType::Node, rm_dirs)
    }

    pub fn cargo(path: String) -> Project {
        let rm_dirs = vec![PathBuf::from("target")];
        Project::new(path, ProjectType::Cargo, rm_dirs)
    }

    pub fn mix(path: String) -> Project {
        let rm_dirs = vec![PathBuf::from("_build"), PathBuf::from("deps")];
        Project::new(path, ProjectType::Mix, rm_dirs)
    }

    pub fn dotnet(path: String) -> Project {
        let rm_dirs = vec![PathBuf::from("bin"), PathBuf::from("obj")];
        Project::new(path, ProjectType::Dotnet, rm_dirs)
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.path, self.project_type)
    }
}