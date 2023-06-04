use std::{process::{Command, Stdio}, io::Write};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use assert_fs::prelude::{PathChild, FileTouch, PathCreateDir};
use predicates::prelude::predicate::str;

#[test]
fn path_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("polykill")?;

    cmd.arg("path/does/not/exist");
    cmd.assert()
        .success()
        .stdout(str::contains("does not exist"));

    Ok(())
}

#[test]
fn is_a_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("polykill")?;

    cmd.arg("Cargo.toml");
    cmd.assert()
        .success()
        .stdout(str::contains("is a file"));

    Ok(())
}

#[test]
fn no_projects_found() -> Result<(), Box<dyn std::error::Error>> {
    let test_dir = assert_fs::TempDir::new()?;
    
    let mut cmd = Command::cargo_bin("polykill")?;

    cmd.args(["--dry-run", test_dir.path().to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(str::contains("No projects found."));

    Ok(())
}

#[test]
fn project_found() -> Result<(), Box<dyn std::error::Error>> {
    let test_dir = assert_fs::TempDir::new()?;
    let test_proj = test_dir.child("test_proj");
    test_proj.create_dir_all()?;
    test_proj.child(".git").touch()?;
    test_proj.child("bin").touch()?;

    
    let mut cmd = Command::cargo_bin("polykill")?;
    cmd.args(["--dry-run", test_dir.path().to_str().unwrap()]);
    
    let mut proc = cmd.stdin(Stdio::piped()).spawn().unwrap();
    proc.stdin.as_mut().unwrap().write_fmt(format_args!("q"))?;
    proc.wait()?;
    
    cmd.assert()
        .success()
        .stdout(str::is_empty());

    test_dir.close()?;
    Ok(())
}