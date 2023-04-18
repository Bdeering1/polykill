use std::path::PathBuf;

use crate::types::ProjectType;

pub struct Project {
    path: String,
    repo_type: ProjectType,
    rm_dirs: Vec<PathBuf>
}

impl Project {
    fn new(path: String, repo_type: ProjectType, rm_dirs: Vec<PathBuf>) -> Project {
        Project { path, repo_type, rm_dirs }
    }
}