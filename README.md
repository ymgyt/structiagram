# Structiagram

Structiagram is a tool to generate Rust struct relation diagram as `mermaid.js` format.

## Installation

---

### Cargo Installation

Install `structiagram` as a CLI executable using `cargo`

```sh
cargo install structiagram
```

## Usage

---

Run `structiagram --help` for the `structiagram` CLI parameter usage.

```sh
$ structiagram --help
structiagram 0.1.2

USAGE:
structiagram [OPTIONS] --dir <DIR>

OPTIONS:
--dir <DIR> Root directory to parse files
-h, --help Print help information
-o, --output <OUTPUT> Output file. default stdout. The '-' is interpreted as stdout
-V, --version Print version information
```

Example usage of generating a diagram

```sh
structiagram --dir src
```

Example usage of generating a diagram and saving the output to a markdown file

```sh
structiagram --dir src --output project_diagram.md
```

## 🪪 License

---

This project is available under the terms of either the [Apache 2.0 license](./LICENSE-APACHE) or the [MIT license](./LICENSE-MIT).
