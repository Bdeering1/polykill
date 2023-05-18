# Polykill

Lightweight command line utility for removing dependencies and build artifacts from unused local projects. Inspired by [npkill](https://www.npmjs.com/package/npkill#usage).

Supported project types:
- Cargo
- Node
- Mix
- .NET Core
- Gradle
- Misc (see "Addional Information")

## Installation

```sh
cargo install polykill
```

## Usage

```sh
polykill [OPTIONS] [DIR]
```

Polykill will recursively search for projects in the provided directory and output a list of all projects found. If no directory is provided, the current directory will be searched.

Move between listed projects using ↓,↑,←,→ and press enter to delete artifacts for the selected project.

Press *q* or *esc* to exit.

**Warning for Node projects:** Some Node applications need their node_modules directory to work and deleting it may break them.

## Options

| Argument       | Description                         |
| -------------- | ----------------------------------- |
| -n, --no-git   | Include projects not tracked by git |
| -v, --verbose  | Verbose output                      |
| -h, --help     | Print help                          |
| -V, --version  | Print version                       |

*--no-git option will slow down project search

## Additional Information

How projects are identified and which directories are used for dependencies and build artifacts:

| Type      | Identifier(s)      | Directories      |
| --------- | ------------------ | ---------------- |
| Node      | package.json       | node_modules     |
| Cargo     | cargo.toml         | target           |
| Mix       | mix.exs            | _build, deps     |
| .NET Core | .csproj            | bin, obj         |
| Gradle    | build.gradle(.kts) | build            |
| Misc      | bin, build, dist   | bin, build, dist |
