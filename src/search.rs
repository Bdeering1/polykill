use std::path::{Path, PathBuf};

use crate::project::Project;

pub fn find_projects(path: &Path) -> Vec<Project> {
    let mut projects = Vec::new();
    let entries = path.read_dir();
    if entries.is_err() { return projects; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let path = entry.unwrap().path();
        if !path.is_dir() { continue; }

        check_for_project(&mut projects, path);
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
            check_for_project(&mut projects, path);
        } else {
            projects.append(&mut find_git_projects(&path));
        }
    }
    projects
}

fn check_for_project(projects: &mut Vec<Project>, path: PathBuf) {
    if is_node(&path) {
        projects.push(Project::node(path));
    } else if is_cargo(&path) {
        projects.push(Project::cargo(path));
    } else if is_mix(&path) {
        projects.push(Project::mix(path));
    } else if is_dotnet(&path) {
        projects.push(Project::dotnet(path));
    } else if is_gradle(&path) {
        projects.push(Project::gradle(path));
    } else {
        projects.append(&mut find_projects(&path));
    }
}

fn is_node(path: &Path) -> bool {
    contains_file(path, "package.json") 
}
fn is_cargo(path: &Path) -> bool {
    contains_file(path, "Cargo.toml")
}
fn is_mix(path: &Path) -> bool {
    contains_file(path, "mix.exs")
}
fn is_dotnet(path: &Path) -> bool {
    contains_file_regex(path, ".csproj")
}
fn is_gradle(path: &Path) -> bool {
    contains_file(path, "build.gradle") || contains_file(path, "build.gradle.kts")
}
fn is_repo(path: &Path) -> bool {
    contains_file(path, ".git")
}

fn contains_file(path: &Path, file: &str) -> bool {
    let res = path.join(file).try_exists();
    if res.is_err() { false } else { res.unwrap() }
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