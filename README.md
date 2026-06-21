# lx

Simple, opinionated replacement for `ls`:

- Formatted list view
- Colored output
- Optional permission strings
- Hidden-file support
- Directory visibility controls
- Simple glob filtering

## Purpose

`lx` shows directories first, then files, with each group sorted alphabetically. It displays file sizes, modification dates, modification times, and color-coded names so directory listings are easy to scan.

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

Default configuration file `~/.config/lx.toml` is created when the app is first run.
CLI flags override configuration settings.

```sh
~/.config/lx.toml
```

Default configuration (only suits a dark terminal theme).

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

## Build

```sh
cargo build
```

The debug binary is written to:

```sh
./target/debug/lx
```

## License

`lx` is licensed under the GNU General Public License v3.0.

## Author

Russell Dickenson + AI coding agents
