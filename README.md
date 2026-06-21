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

CLI flags override configuration settings.

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

Configuration file `~/.config/lx.toml`. Created when `lx` is first run.

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

### Color codes

Color values are ANSI Select Graphic Rendition (SGR) parameters. For example:

- `"32"` = green foreground
- `"34"` = blue foreground
- `"33"` = yellow foreground
- `"38;5;250"` = 256-color foreground color 250

You can use standard foreground colors like `30`-`37`, bright foreground colors like `90`-`97`, or 256-color foreground values in the form `38;5;N`, where `N` is `0`-`255`.

For more options, see the ANSI escape code SGR color reference:
https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters

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
