use std::{process::{Command, Stdio}, io::Write};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
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
    let mut cmd = Command::cargo_bin("polykill")?;

    cmd.args(["--dry-run", "tests/test_dirs/empty"]);
    cmd.assert()
        .success()
        .stdout(str::contains("No projects found."));

    Ok(())
}

#[test]
fn project_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("polykill")?;
    cmd.args(["--dry-run", "tests/test_dirs/contains_proj"]);

    let mut proc = cmd.stdin(Stdio::piped()).spawn().unwrap();
    proc.stdin.as_mut().unwrap().write_fmt(format_args!("q"))?;
    proc.wait()?;

    cmd.assert()
        .success()
        .stdout(str::is_empty());

    Ok(())
}