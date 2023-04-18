use std::path::Path;

mod project;
mod search;
mod project_types;

fn main() {
    let projects = search::find_git_projects(Path::new("../../"));
    for project in projects {
        println!("{}", project);
    }
}
