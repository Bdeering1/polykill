use std::path::Path;

#[derive(Debug)]
pub enum ProjectType {
    Node,
    Cargo,
    Dotnet,
    Mix
}

pub fn is_node(path: &Path) -> bool {
    contains_file(path, "package.json") 
}

pub fn is_cargo(path: &Path) -> bool {
    contains_file(path, "Cargo.toml")
}

pub fn is_mix(path: &Path) -> bool {
    contains_file(path, "mix.exs")
}

pub fn is_dotnet(path: &Path) -> bool {
    contains_file_regex(path, ".csproj")
}

pub fn is_repo(path: &Path) -> bool {
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