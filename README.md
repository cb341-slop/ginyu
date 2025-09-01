# Ginyu

A lightweight git CLI wrapper that provides intuitive aliases for common git operations.

## Installation

### From source
```bash
git clone https://github.com/CuddlyBunion341/ginyu
cd ginyu
cargo install --path .
```

### From crates.io
```bash
cargo install ginyu
```

## Quick Start

```bash
# List modified files
ginyu modified

# Format only modified files
ginyu prettier modified

# Diff files in feature branch
ginyu diff branch-files main

# Lint added files
ginyu lint added
```

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `ginyu added` | `ginyu a` | Show added but not staged files |
| `ginyu modified` | `ginyu m` | Show modified but not staged files |
| `ginyu diff [files]` | `ginyu d [files]` | Show diff for file set |
| `ginyu prettier [files]` | `ginyu p [files]` | Run prettier on file set |
| `ginyu lint [files]` | `ginyu l [files]` | Run linter on file set |

Run `ginyu --help` for full command list.

## License

MIT
