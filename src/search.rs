use std::path::{Path, PathBuf};

use crate::project::Project;

pub fn find_projects(path: &Path, max_depth: u32) -> Vec<Project> {
    if max_depth == 0 { return Vec::new(); }

    let mut projects = Vec::new();
    let entries = path.read_dir();
    if entries.is_err() { return projects; }

    for entry in entries.unwrap() {
        if entry.is_err() { continue; }

        let path = entry.unwrap().path();
        if !path.is_dir() { continue; }

        check_for_project(&mut projects, path, max_depth - 1);
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
            check_for_project(&mut projects, path, 1);
        } else {
            projects.append(&mut find_git_projects(&path));
        }
    }
    projects
}

fn check_for_project(projects: &mut Vec<Project>, path: PathBuf, max_depth: u32) {
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
    } else if is_composer(&path) {
        projects.push(Project::composer(path));
    } else if let Some(rm_dir) = is_misc_project(&path) {
        projects.push(Project::misc(path.to_owned(), vec![path.join(rm_dir)]));
    } else {
        projects.append(&mut find_projects(&path, max_depth));
    }
}

fn is_node(path: &Path) -> bool {
    contains_entry(path, "package.json") 
}
fn is_cargo(path: &Path) -> bool {
    contains_entry(path, "Cargo.toml")
}
fn is_mix(path: &Path) -> bool {
    contains_entry(path, "mix.exs")
}
fn is_dotnet(path: &Path) -> bool {
    contains_file_regex(path, ".csproj")
}
fn is_gradle(path: &Path) -> bool {
    contains_entry(path, "build.gradle") || contains_entry(path, "build.gradle.kts")
}
fn is_composer(path: &Path) -> bool {
    contains_entry(path, "composer.json")
}
fn is_misc_project(path: &Path) -> Option<PathBuf> {
    const MISC_DIRS: [&str; 3] = ["bin", "build", "dist"];

    for dir in MISC_DIRS {
        if contains_entry(path, dir) {
            return Some(PathBuf::from(dir));
        }
    }
    None
}
fn is_repo(path: &Path) -> bool {
    contains_entry(path, ".git")
}

fn contains_entry(path: &Path, entry: &str) -> bool {
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