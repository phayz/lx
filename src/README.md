# lx

`lx` is a Rust command-line app for listing directory contents. It is intended as a small alternative to `ls`, with a formatted list view, colored output, optional permission strings, hidden-file support, directory visibility controls, and simple glob filtering.

## Purpose

`lx` shows directories first, then files, with each group sorted alphabetically. It displays file sizes, modification dates, modification times, and color-coded names so directory listings are easy to scan.

## Build

```sh
cargo build
```

The debug binary is written to:

```sh
./target/debug/lx
```

## Usage

List the current directory:

```sh
lx
```

List another directory:

```sh
lx src
lx ..
```

Show hidden files:

```sh
lx --all
lx -a
```

Show permission strings:

```sh
lx --permissions
lx -p
```

Hide directories:

```sh
lx --no-showdir
```

Show directories when configuration hides them:

```sh
lx --showdir
```

Show the app version:

```sh
lx --version
lx -v
```

Use a glob pattern:

```sh
lx '*.md'
lx 'src/*.rs'
```

## Configuration

Configuration is read from:

```sh
~/.config/lx.toml
```

Example:

```toml
[settings]
all = false
permissions = false
showdir = true

[colors]
size = "32"
date = "34"
time = "34"
file = "38;5;250"
dir = "33"
exec = "32"
```

CLI flags override configuration settings.

## License

`lx` is licensed under the GNU General Public License v3.0.

## Author

Russell Dickenson + AI coding agents
