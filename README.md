# Polykill

Lightweight utility for removing unwanted dependencies and build artifacts from local projects. Inspired by [npkill](https://www.npmjs.com/package/npkill).

Supported project types:
- Cargo
- Node
- Mix
- .NET Core
- Gradle
- Composer
- Misc. (see "Addional Information")

## Installation

```sh
cargo install polykill
```

## Usage

```sh
polykill [OPTIONS] [DIR]
```

Polykill will recursively search for projects in the provided directory and output a list of all projects found. If no directory is provided, the current directory will be searched.

When the search has completed, navigate the menu using the following controls:

| Key Bind | Action           |
| -------- | ---------------- |
| ↓, ↑, ←, →, h, j, k, l | make selection        |
| enter, del    | remove artifacts |
| esc, q   | exit             |

Removing artifacts involves removing the build/dependency directories associated with a project type.

**Warning for Node projects:** Some Node applications need their node_modules directory to work and deleting it may break them.

## Options

| Argument         | Description                                   |
| ---------------- | --------------------------------------------- |
| -v, --verbose    | Verbose output                                |
| -s, --skip-empty | Hide projects with zero possible disk savings |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;--no-vcs     | Include projects without version control (will slow down search) |
| -u, --unsorted   | Don't sort projects                           |
| -h, --help       | Print help                                    |
| -V, --version    | Print version                                 |

*supported version control systems are: git, svn, and mercurial

## Additional Information

How projects are identified and which directories are used for dependencies and build artifacts:

| Type      | Identifier(s)      | Directories      |
| --------- | ------------------ | ---------------- |
| Node      | package.json       | node_modules     |
| Cargo     | cargo.toml         | target           |
| Mix       | mix.exs            | _build, deps     |
| .NET Core | .csproj            | bin, obj         |
| Gradle    | build.gradle(.kts) | build            |
| Composer  | composer.json      | vendor           |
| Misc.     | bin, build, dist   | bin, build, dist |
