# Polykill

[![Crates.io](https://img.shields.io/crates/v/polykill?style=flat-square&color=peru)](https://crates.io/crates/polykill)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/polykill?style=flat-square&color=mediumseagreen)](https://crates.io/crates/polykill)
[![Crates.io Size](https://img.shields.io/crates/size/polykill?style=flat-square&color=steelblue)](https://crates.io/crates/polykill)

*Like [polyfill](https://developer.mozilla.org/en-US/docs/Glossary/Polyfill) - but more violent*

Lightweight command line utility for removing unwanted dependencies and build artifacts from local projects. Inspired by [npkill](https://www.npmjs.com/package/npkill).

Supported project types:
- Node
- Cargo
- .NET
- Go
- Gradle
- Mix
- Composer
- Misc. (see "Additional Information")

## Installation

**Cargo**
```sh
cargo install polykill
```

**Homebrew**
```sh
brew tap bdeering1/polykill
brew install polykill
```

**Nix (Flake)**

```sh
nix shell github:Bdeering1/polykill
# OR
nix run github:Bdeering1/polykill
```

## Usage

```sh
polykill [OPTIONS] [DIR]
```

Polykill will recursively search for projects in the provided directory and output a list of all projects found. If no directory is provided, the current directory will be searched.

When the search has completed, navigate the menu using the following controls:

| Key Bind   | Action           |
| ---------- | ---------------- |
| ↓, ↑, ←, →, h, j, k, l | select project |
| enter, del | remove artifacts |
| esc, q     | exit             |

**Warning for Node projects:** Some Node applications need their node_modules directory to work and deleting it may break them.

## Options

| Argument         | Description                                   |
| ---------------- | --------------------------------------------- |
| -v, --verbose    | Verbose output                                |
| -s, --skip-empty | Hide projects with zero possible disk savings |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;--no-vcs | Include projects without version control (will slow down search) |
| -u, --unsorted   | Don't sort projects                           |
| -h, --help       | Print help                                    |
| -V, --version    | Print version                                 |

*supported version control systems are: git, svn, and mercurial

## Additional Information

How projects are identified and which files or directories will be removed:

| Type      | Identifier(s)      | Artifacts        |
| --------- | ------------------ | ---------------- |
| Node      | package.json       | node_modules     |
| Cargo     | cargo.toml         | target           |
| .NET      | .csproj            | bin, obj         |
| Go        | go.mod             | dir(.exe), dir.test(.exe) |
| Gradle    | build.gradle(.kts) | build            |
| Mix       | mix.exs            | _build, deps     |
| Composer  | composer.json      | vendor           |
| Misc.     | bin, build, dist   | bin, build, dist |

*dir for go projects is the name of the project directory
