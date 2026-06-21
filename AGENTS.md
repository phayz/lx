# ll - Rust ls Alternative

## Build & Run
- Build: `cargo build`
- Binary: `./target/debug/ll`

## CLI Flags
- First positional arg: directory path (e.g., `ll src`, `ll ..`)
- `-a, --all` - show hidden files
- `-p, --permissions` - show permission string
- `-v, --version` - show version
- `--showdir` - show directories (default: true, use `--no-showdir` to hide)

## Configuration
- Colors defined in `~/.config/ll.toml` (not in repo)
- Settings (all, permissions, showdir) also configurable in config file
- CLI flags always override config settings

## Project Structure
- Single binary, no workspace
- Dependencies: clap, chrono, glob, toml