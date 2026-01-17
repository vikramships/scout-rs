<div align="center">

# scout-rs

**Fast file finder for AI agents**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## What is scout-rs?

A blazing-fast file finder that returns **JSON output** - designed for AI agents, not humans.

```bash
$ scout find "src/**/*.rs"
[{"path": "src/main.rs", "size": 1524}, {"path": "src/utils.rs", "size": 892}]
```

---

## Features

- **⚡ Fast** - ~6ms for 100 files
- **📦 JSON output** - Machine-readable
- **🔍 Glob patterns** - `**/*.rs`, `src/**/*.tsx`
- **🔎 Content search** - Grep-like with line numbers
- **📁 Directory listing** - Fast enumeration
- **🚫 Exclude patterns** - Skip node_modules, target, etc.

---

## Install

```bash
git clone https://github.com/vikramships/scout-rs.git
cd scout-rs
cargo build --release
cp target/release/scout ~/.local/bin/
```

---

## Usage

### Find files
```bash
scout find "src/**/*.rs" --limit 50
scout find "*.tsx" --root /path/to/project
scout find "**/*.json" --exclude "node_modules,target"
```

### List directory
```bash
scout list --limit 100
scout list --exclude "target,node_modules"
```

### Search contents
```bash
scout search "function_name" --ext "ts,tsx"
scout search "TODO" --root /path/to/project
```

---

## Output

### Find/List
```json
[{"path": "src/main.rs", "size": 1524}]
```

### Search
```json
[{"path": "src/main.rs", "line": 42, "content": "fn main() {"}]
```

---

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `-r, --root <PATH>` | Root directory | Current |
| `--gitignore` / `--no-gitignore` | Respect .gitignore | true |
| `--hidden` / `--no-hidden` | Show hidden files | true |
| `--exclude <PATTERNS>` | Exclude patterns | none |
| `--limit <N>` | Max results | varies |

---

## Performance

| Command | Time |
|---------|------|
| find (100 files) | ~6ms |
| list (1000 files) | ~6ms |
| search (100 matches) | ~89ms |

---

## Why not fd/rg?

| | scout | fd | rg |
|---|---|---|---|
| JSON output | ✅ | ❌ | ❌ |
| Designed for AI | ✅ | ❌ | ❌ |

---

## License

MIT

---

<div align="center">

https://github.com/vikramships/scout-rs

</div>
