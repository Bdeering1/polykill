# Polykill

Lightweight command line utility for removing dependencies and build artifacts from unused local projects. Inspired by [npkill](https://www.npmjs.com/package/npkill#usage).

Supported project types:
- Cargo
- Node
- Mix
- .NET Core
- Gradle

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

## Additional Information

Directories removed for each project type:

| Type      | Directories  |
| --------- | ------------ |
| Node      | node_modules |
| Cargo     | target       |
| Mix       | _build, deps |
| .NET Core | bin, obj     |
| Gradle    | build        |
