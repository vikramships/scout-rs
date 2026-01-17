<div align="center">

```
    ____
   / __ \____ _(__)_  ___
  / /_/ / __ `/ / / / / _ \
 / ____/ /_/ / / / / /  __/
/_/    \__,_/_/ /_/_/\___/

```
**Fast file finder for AI agents**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## What is Scout?

**Scout** is a blazing-fast file finder and searcher built specifically for AI agents. Unlike tools designed for humans (fd, rg, fzf), Scout returns **structured JSON** optimized for machine consumption.

AI agents don't need colored output or interactive TUIs—they need raw data, fast. Scout gives them exactly that.

---

## Features

- **⚡ Blazing fast** - ~6ms for finding 100 files
- **📦 JSON output** - Machine-readable, easy to parse
- **🔍 Pattern matching** - Glob-style patterns (`**/*.rs`, `src/**/*.tsx`)
- **🔎 Content search** - Grep-like search with line numbers
- **📁 Directory listing** - Fast file enumeration with sizes
- **🚫 Exclude patterns** - Skip node_modules, target, .git, etc.
- **👻 Hidden files** - Access all files by default (dotfiles included)
- **📝 .gitignore aware** - Respect or ignore .gitignore as needed

---

## Installation

### Cargo
```bash
cargo install scout-cli
```

### Build from source
```bash
git clone https://github.com/vikramships/scout.git
cd scout
cargo build --release
cp target/release/scout ~/.local/bin/
```

---

## Usage

### Find files by pattern
```bash
scout find "src/**/*.rs" --limit 50
scout find "*.tsx" --root /path/to/project
scout find "**/*.json" --exclude "node_modules,target"
```

### List directory
```bash
scout list --limit 100
scout list --exclude "target,node_modules"
scout list --no-hidden --no-gitignore
```

### Search file contents
```bash
scout search "function_name" --ext "ts,tsx" --limit 20
scout search "TODO" --root /path/to/project
```

---

## JSON Output

### Find command
```json
[
  {"path": "src/main.rs", "size": 1524},
  {"path": "src/utils.rs", "size": 892}
]
```

### Search command
```json
[
  {"path": "src/main.rs", "line": 42, "content": "fn main() {"},
  {"path": "src/utils.rs", "line": 15, "content": "pub fn helper() {"}
]
```

### List command
```json
[
  {"path": "Cargo.toml", "size": 619},
  {"path": "src/main.rs", "size": 1524}
]
```

---

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `-r, --root <PATH>` | Root directory | Current directory |
| `--gitignore` / `--no-gitignore` | Respect .gitignore | true |
| `--hidden` / `--no-hidden` | Show hidden files | true |
| `--exclude <PATTERNS>` | Comma-separated patterns to exclude | none |
| `--limit <N>` | Maximum results | varies by command |

---

## Performance

Benchmarked on a MacBook Pro M1, searching a codebase with 10,000+ files:

| Command | Time |
|---------|------|
| `find` (100 files) | ~6ms |
| `list` (1000 files) | ~6ms |
| `search` (100 matches) | ~89ms |

---

## Why Scout?

AI agents work differently than humans:

| Human Tools | AI Tools |
|-------------|----------|
| Colored output | Structured JSON |
| Interactive TUI | Command-line args |
| Pretty formatting | Raw data |
| Fuzzy matching | Exact patterns |
| Key bindings | Simple stdin/stdout |

**Scout is built for AI agents, not humans.**

---

## Comparison

| Feature | Scout | fd | rg |
|---------|-------|----|----|
| JSON output | ✅ | ❌ | ❌ |
| Glob patterns | ✅ | ✅ | Partial |
| Content search | ✅ | ❌ | ✅ |
| Machine-readable | ✅ | ❌ | ❌ |
| Designed for AI | ✅ | ❌ | ❌ |

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

## Author

Made by [@vikramships](https://github.com/vikramships)

---

<div align="center">

**⭐ Star us on GitHub! ⭐**

[GitHub](https://github.com/vikramships/scout) • [Issues](https://github.com/vikramships/scout/issues) • [Discussions](https://github.com/vikramships/scout/discussions)

</div>
