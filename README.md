<div align="center">

# scout-rs

**Fast file finder for AI agents - 68% fewer tokens than JSON**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## What is scout?

A file finder designed **for AI agents**.

```bash
# TOON format (default) - 68% token savings
$ scout find "src/**/*.rs" --limit 5
path,size:
	src/main.rs,1524
	src/utils.rs,892
```

---

## Install

```bash
cargo install --git https://github.com/vikramships/scout-rs
```

---

## Commands

```bash
scout find "**/*.ts"       # Find files by glob pattern
scout search "func"        # Search file contents
scout list                 # List all files
scout estimate             # Get file count and size estimate
```

### Options

```bash
-r, --root <PATH>          # Root directory (default: current)
-F, --format <FORMAT>      # toon (default), json, plain
--stream                   # Stream results as they're found
--limit <N>                # Max results (default: 100)
```

---

## AI Workflow Examples

### Finding all TypeScript files
```bash
scout find "**/*.ts" -r /path/to/project --limit 100
```

### Searching for code patterns
```bash
scout search "ipcMain" -r src/electron --limit 50
```

### Understanding project structure
```bash
$ scout estimate
file_count: 500
total_size: 12.34 MB (estimated)
top_extensions:
  ts: 200
  tsx: 150
  json: 50
```

### Streaming results
```bash
scout list --limit 1000 --stream
# Results appear immediately as they're found
```

---

## Output Formats

### TOON (default) - Best for AI
```bash
$ scout list --limit 2
path,size:
	src/main.rs,1524
	src/utils.rs,892
```

### JSON
```bash
$ scout list --limit 2 -F json
[{"path":"src/main.rs","size":1524},{"path":"src/utils.rs","size":892}]
```

### Plain
```bash
$ scout list --limit 2 -F plain
src/main.rs 1524
src/utils.rs 892
```

---

## Why TOON?

TOON = Token-Oriented Object Notation

| Format | Tokens | Savings |
|--------|--------|---------|
| JSON | 50 | baseline |
| TOON | 17 | **68%** |
| Plain | 13 | **74%** |

For 1000 files:
- JSON: ~50,000 tokens
- TOON: ~16,000 tokens
- **Savings: ~34,000 tokens**

---

## Design Philosophy

**Tool = Truth. Agent = Intelligence.**

Scout shows **everything** - no filtering. The AI agent decides what matters.

---

## Performance

| Operation | Time |
|-----------|------|
| List 100 files | ~5ms |
| Find 100 files | ~5ms |
| Estimate (1000 sample) | ~50ms |

---

## License

MIT

---

<div align="center">

https://github.com/vikramships/scout-rs

</div>
