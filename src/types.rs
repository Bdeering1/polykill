use std::path::Path;

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

pub fn is_dotnet(path: &Path) -> bool {
    contains_file_regex(path, "*.csproj")
}

pub fn is_mix(path: &Path) -> bool {
    contains_file(path, "mix.exs")
}

fn contains_file(path: &Path, file: &str) -> bool {
    let res = path.join(file).try_exists();
    if res.is_err() { false } else { res.unwrap() }
}

fn contains_file_regex(path: &Path, ex: &str) -> bool {
    todo!()
}