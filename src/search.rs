use std::path::{Path, PathBuf};

use crate::project::{self, Project};

pub fn find_projects(path: &Path, max_depth: u32) -> Vec<Project> {
    if max_depth == 0 { return Vec::new(); }

    let mut projects = Vec::new();
    let entries = path.read_dir();
    if entries.is_err() { return projects; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let path = entry.unwrap().path();
        if !path.is_dir() || path.file_name().unwrap().as_encoded_bytes()[0] == b'.' { continue; }

        if let Some(project) = check_for_project(path.clone()) {
            projects.push(project);
      } else {
            projects.append(&mut find_projects(&path, max_depth - 1));
        }
    }
    projects
}

pub fn find_git_projects(path: &Path) -> Vec<Project> {
    let mut projects = Vec::new();
    let entries = path.read_dir();
    if entries.is_err() { return projects; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let path = entry.unwrap().path();
        if !path.is_dir() || path.file_name().unwrap().as_encoded_bytes()[0] == b'.' { continue; }

        if is_repo(&path) {
            if let Some(project) = check_for_project(path.clone()) {
                projects.push(project);
            } else {
                projects.append(&mut find_projects(&path, 2));
            }
        } else {
            projects.append(&mut find_git_projects(&path));
        }
    }
    projects
}

fn check_for_project(path: PathBuf) -> Option<Project> {
    for (pt, idents) in project::PROJECT_IDENTIFIERS {
        for ident in *idents {
            if contains_entry(&path, ident) { // .csproj is special case (should use regex)
                return Some(project::PROJECT_CONSTRUCTORS.get(pt).unwrap()(path));
            }
        }
    }
    None
}

fn is_repo(path: &Path) -> bool {
    contains_entry(path, ".git")
    || contains_entry(path, ".svn")
    || contains_entry(path, ".hg")
}

pub fn contains_entry(path: &Path, entry: &str) -> bool {
    let res = path.join(entry).try_exists();
    if let Ok(val) = res { val } else { false }
}

fn contains_file_regex(path: &Path, pattern: &str) -> bool {
    let entries = path.read_dir();
    if entries.is_err() { return false; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();

        if file_name.ends_with(pattern) { return true; }
    }
    false
}
