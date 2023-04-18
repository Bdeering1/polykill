use std::path::Path;

use crate::project::Project;
use crate::project_types::{is_repo, is_node, is_cargo, is_mix, is_dotnet, ProjectType};

pub fn find_projects(path: &Path) -> Vec<Project> {
    let mut projects = Vec::new();
    let entries = path.read_dir();
    if entries.is_err() { return projects; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let path = entry.unwrap().path();
        if !path.is_dir() { continue; }

        let path_name = path.to_str().unwrap().to_string();
        if is_node(&path) {
            projects.push(Project::new(path_name, ProjectType::Node, vec![]));
        } else if is_cargo(&path) {
            projects.push(Project::new(path_name, ProjectType::Cargo, vec![]));
        } else if is_mix(&path) {
            projects.push(Project::new(path_name, ProjectType::Mix, vec![]));
        } else if is_dotnet(&path) {
            projects.push(Project::new(path_name, ProjectType::Dotnet, vec![]));
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
            let path_name = path.to_str().unwrap().to_string();
            if is_node(&path) {
                projects.push(Project::node(path_name));
            } else if is_cargo(&path) {
                projects.push(Project::cargo(path_name));
            } else if is_mix(&path) {
                projects.push(Project::mix(path_name));
            } else if is_dotnet(&path) {
                projects.push(Project::dotnet(path_name));
            } else {
                projects.append(&mut find_projects(&path));
            }
        } else {
            projects.append(&mut find_git_projects(&path));
        }
    }
    projects
}