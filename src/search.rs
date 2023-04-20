use std::path::Path;

use crate::project::Project;
use crate::project_type::{is_repo, is_node, is_cargo, is_mix, is_dotnet};

pub fn find_projects(path: &Path) -> Vec<Project> {
    let mut projects = Vec::new();
    let entries = path.read_dir();
    if entries.is_err() { return projects; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let path = entry.unwrap().path();
        if !path.is_dir() { continue; }

        if is_node(&path) {
            projects.push(Project::node(path));
        } else if is_cargo(&path) {
            projects.push(Project::cargo(path));
        } else if is_mix(&path) {
            projects.push(Project::mix(path));
        } else if is_dotnet(&path) {
            projects.push(Project::dotnet(path));
        } else {
            projects.append(&mut find_projects(&path));
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
        if !path.is_dir() { continue; }

        if is_repo(&path) {
            if is_node(&path) {
                projects.push(Project::node(path));
            } else if is_cargo(&path) {
                projects.push(Project::cargo(path));
            } else if is_mix(&path) {
                projects.push(Project::mix(path));
            } else if is_dotnet(&path) {
                projects.push(Project::dotnet(path));
            } else {
                projects.append(&mut find_projects(&path));
            }
        } else {
            projects.append(&mut find_git_projects(&path));
        }
    }
    projects
}