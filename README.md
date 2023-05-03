# Polykill

Command line tool for removing dependencies and build artifacts from unused local projects. Inspired by [npkill](https://www.npmjs.com/package/npkill#usage).

Supported project types:
- Cargo
- Node
- Mix
- .NET Core

## Installation

```sh
cargo install polykill
```

## Usage

```sh
polykill /my-projects-directory
```

Polykill will recursively search for supported projects in the provided directory and output a list of all projects found.

Move between listed projects using ↓ ↑, and press enter to delete artifacts for the selected project.

Press *q* or *esc* to exit.

**Warning for Node projects:** Some Node applications need their node_modules directory to work and deleting them may break them.

## Project Information

Directories removed for each project type

| Type      | Directories  |
| --------- | ------------ |
| Node      | node_modules |
| Cargo     | target       |
| Mix       | _build, deps |
| .NET Core | bin, obj     |
