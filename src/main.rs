use chrono::TimeZone;
use clap::{ArgAction, Parser};
use glob::Pattern;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

#[derive(Parser)]
#[command(name = "lx", about = "List directory contents")]
struct Args {
    #[arg(default_value = ".", hide_default_value = true)]
    path: String,

    #[arg(short, long, help = "Show hidden files")]
    all: bool,

    #[arg(short, long, help = "Show permissions")]
    permissions: bool,

    #[arg(short = 'v', long, help = "Show version")]
    version: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Show directories")]
    showdir: bool,

    #[arg(long = "no-showdir", action = ArgAction::SetTrue, help = "Hide directories")]
    no_showdir: bool,
}

#[derive(Clone)]
struct Entry {
    name: String,
    sort_name: String,
    is_dir: bool,
    is_exec: bool,
    size: u64,
    modified: std::time::SystemTime,
    permissions: String,
}

#[derive(Clone)]
struct Colors {
    size: String,
    date: String,
    time: String,
    file: String,
    dir: String,
    exec: String,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            size: "32".to_string(),
            date: "34".to_string(),
            time: "34".to_string(),
            file: "38;5;250".to_string(),
            dir: "33".to_string(),
            exec: "32".to_string(),
        }
    }
}

#[derive(Clone, Default)]
struct Settings {
    all: bool,
    permissions: bool,
    showdir: bool,
}

#[derive(Clone, Default)]
struct Config {
    colors: Colors,
    settings: Settings,
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("lx.toml")
}

fn load_config() -> Config {
    let config_path = get_config_path();

    let mut config = Config {
        settings: Settings {
            showdir: true,
            ..Settings::default()
        },
        ..Config::default()
    };

    if !config_path.exists() {
        return config;
    }

    let content = fs::read_to_string(&config_path).unwrap_or_default();

    if let Ok(value) = content.parse::<toml::Value>() {
        if let Some(table) = value.get("colors").and_then(|v| v.as_table()) {
            if let Some(v) = table.get("size") {
                config.colors.size = v.as_str().unwrap_or("32").to_string();
            }
            if let Some(v) = table.get("date") {
                config.colors.date = v.as_str().unwrap_or("34").to_string();
            }
            if let Some(v) = table.get("time") {
                config.colors.time = v.as_str().unwrap_or("34").to_string();
            }
            if let Some(v) = table.get("file") {
                config.colors.file = v.as_str().unwrap_or("38;5;250").to_string();
            }
            if let Some(v) = table.get("dir") {
                config.colors.dir = v.as_str().unwrap_or("33").to_string();
            }
            if let Some(v) = table.get("exec") {
                config.colors.exec = v.as_str().unwrap_or("32").to_string();
            }
        }

        if let Some(table) = value.get("settings").and_then(|v| v.as_table()) {
            if let Some(v) = table.get("all") {
                config.settings.all = v.as_bool().unwrap_or(false);
            }
            if let Some(v) = table.get("permissions") {
                config.settings.permissions = v.as_bool().unwrap_or(false);
            }
            if let Some(v) = table.get("showdir") {
                config.settings.showdir = v.as_bool().unwrap_or(true);
            }
        }
    }

    config
}

fn has_glob_chars(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

fn parse_directory_and_pattern(input: &str) -> (PathBuf, Option<Pattern>) {
    if has_glob_chars(input) {
        let path = Path::new(input);
        if let Some(parent) = path.parent() {
            if parent.as_os_str().is_empty() {
                (PathBuf::from("."), Pattern::new(input).ok())
            } else {
                let pattern = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .and_then(|pattern_str| Pattern::new(pattern_str).ok());
                (PathBuf::from(parent), pattern)
            }
        } else {
            (PathBuf::from("."), Pattern::new(input).ok())
        }
    } else {
        (PathBuf::from(input), None)
    }
}

fn strip_trailing_zero(s: &str) -> String {
    if let Some(pos) = s.find('.') {
        if &s[pos + 1..] == "0" {
            return s[..pos].to_string();
        }
    }
    s.to_string()
}

fn format_size(bytes: u64, is_dir: bool) -> String {
    if is_dir {
        return "-".to_string();
    }

    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!(
            "{}T",
            strip_trailing_zero(&format!("{:.1}", bytes as f64 / TB as f64))
        )
    } else if bytes >= GB {
        format!(
            "{}G",
            strip_trailing_zero(&format!("{:.1}", bytes as f64 / GB as f64))
        )
    } else if bytes >= MB {
        format!(
            "{}M",
            strip_trailing_zero(&format!("{:.1}", bytes as f64 / MB as f64))
        )
    } else if bytes >= KB {
        format!(
            "{}K",
            strip_trailing_zero(&format!("{:.1}", bytes as f64 / KB as f64))
        )
    } else {
        format!("{}B", bytes)
    }
}

fn format_date(time: std::time::SystemTime) -> String {
    let secs = timestamp_seconds(time);
    match chrono::Local.timestamp_opt(secs, 0).single() {
        Some(datetime) => datetime.format("%-d %b %Y").to_string(),
        None => "?".to_string(),
    }
}

fn format_time(time: std::time::SystemTime) -> String {
    let secs = timestamp_seconds(time);
    match chrono::Local.timestamp_opt(secs, 0).single() {
        Some(datetime) => datetime.format("%-I:%M %p").to_string(),
        None => "?".to_string(),
    }
}

fn timestamp_seconds(time: std::time::SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    }
}

fn get_mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::symlink_metadata(path)
        .map(|m| m.permissions().mode())
        .unwrap_or(0)
}

fn get_exec_mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|m| m.permissions().mode())
        .unwrap_or(0)
}

fn get_permissions(mode: u32) -> String {
    let file_type = if mode & 0o170000 == 0o040000 {
        'd'
    } else if mode & 0o170000 == 0o120000 {
        'l'
    } else {
        '-'
    };

    let owner = (mode >> 6) & 0o7;
    let group = (mode >> 3) & 0o7;
    let other = mode & 0o7;

    let perm = |bits: u32| {
        let r = if bits & 0o4 != 0 { 'r' } else { '-' };
        let w = if bits & 0o2 != 0 { 'w' } else { '-' };
        let x = if bits & 0o1 != 0 { 'x' } else { '-' };
        format!("{}{}{}", r, w, x)
    };

    format!("{}{}{}{}", file_type, perm(owner), perm(group), perm(other))
}

fn print_colored(text: &str, color_code: &str) {
    print!("\x1b[{}m{}\x1b[0m", color_code, text);
}

fn print_entry(entry: &Entry, show_permissions: bool, colors: &Colors) {
    let size_str = format_size(entry.size, entry.is_dir);
    let date_str = format_date(entry.modified);
    let time_str = format_time(entry.modified);

    if show_permissions {
        print!("{:>10} ", entry.permissions);
    }

    print_colored(&format!("{:>8} ", size_str), &colors.size);
    print_colored(&format!("{:>11} ", date_str), &colors.date);
    print_colored(&format!("{:>8} ", time_str), &colors.time);

    if entry.is_dir {
        print_colored(&entry.name, &colors.dir);
    } else if entry.is_exec {
        print_colored(&entry.name, &colors.exec);
    } else {
        print_colored(&entry.name, &colors.file);
    }

    println!();
}

fn show_single_file(path: &Path, show_permissions: bool, colors: &Colors) {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("lx: cannot access '{}': {}", path.display(), e);
            std::process::exit(1);
        }
    };

    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let is_dir = metadata.is_dir();
    let size = if is_dir { 0 } else { metadata.len() };
    let modified = match metadata.modified() {
        Ok(modified) => modified,
        Err(e) => {
            eprintln!(
                "lx: cannot read modified time for '{}': {}",
                path.display(),
                e
            );
            std::process::exit(1);
        }
    };
    let mode = get_mode(path);
    let is_exec = !is_dir && (get_exec_mode(path) & 0o111) != 0;

    let entry = Entry {
        sort_name: name.to_lowercase(),
        name,
        is_dir,
        is_exec,
        size,
        modified,
        permissions: get_permissions(mode),
    };

    print_entry(&entry, show_permissions, colors);
}

fn entry_from_dir_entry(dir_entry: fs::DirEntry) -> Option<Entry> {
    let name = dir_entry.file_name().to_string_lossy().to_string();
    let path = dir_entry.path();
    let metadata = match dir_entry.metadata() {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("lx: cannot access '{}': {}", path.display(), e);
            return None;
        }
    };
    let modified = match metadata.modified() {
        Ok(modified) => modified,
        Err(e) => {
            eprintln!(
                "lx: cannot read modified time for '{}': {}",
                path.display(),
                e
            );
            return None;
        }
    };
    let is_dir = metadata.is_dir();
    let mode = get_mode(&path);
    let is_exec = !is_dir && (get_exec_mode(&path) & 0o111) != 0;
    let size = if is_dir { 0 } else { metadata.len() };

    Some(Entry {
        sort_name: name.to_lowercase(),
        name,
        is_dir,
        is_exec,
        size,
        modified,
        permissions: get_permissions(mode),
    })
}

fn collect_entries(path: &Path, pattern: Option<&Pattern>, show_all: bool) -> Vec<Entry> {
    let read_dir = match fs::read_dir(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("lx: cannot access '{}': {}", path.display(), e);
            std::process::exit(1);
        }
    };

    let mut entries: Vec<Entry> = Vec::new();

    for dir_entry in read_dir {
        let dir_entry = match dir_entry {
            Ok(dir_entry) => dir_entry,
            Err(e) => {
                eprintln!(
                    "lx: cannot read directory entry in '{}': {}",
                    path.display(),
                    e
                );
                continue;
            }
        };
        let name = dir_entry.file_name().to_string_lossy().to_string();

        if !show_all && name.starts_with('.') {
            continue;
        }

        if let Some(pat) = pattern {
            if !pat.matches(&name) {
                continue;
            }
        }

        if let Some(entry) = entry_from_dir_entry(dir_entry) {
            entries.push(entry);
        }
    }

    entries
}

fn print_entries(entries: Vec<Entry>, show_dirs: bool, show_permissions: bool, colors: &Colors) {
    let (mut dirs, mut files): (Vec<Entry>, Vec<Entry>) =
        entries.into_iter().partition(|e| e.is_dir);

    dirs.sort_by(|a, b| a.sort_name.cmp(&b.sort_name));
    files.sort_by(|a, b| a.sort_name.cmp(&b.sort_name));

    let all_entries: Vec<Entry> = if show_dirs {
        dirs.into_iter().chain(files).collect()
    } else {
        files
    };

    for entry in &all_entries {
        print_entry(entry, show_permissions, colors);
    }
}

fn list_directory(
    path: &Path,
    pattern: Option<&Pattern>,
    show_all: bool,
    show_permissions: bool,
    show_dirs: bool,
    colors: &Colors,
) {
    let entries = collect_entries(path, pattern, show_all);
    print_entries(entries, show_dirs, show_permissions, colors);
}

fn resolve_show_dirs(args: &Args, settings: &Settings) -> bool {
    if args.no_showdir {
        false
    } else if args.showdir {
        true
    } else {
        settings.showdir
    }
}

fn main() {
    let args = Args::parse();

    if args.version {
        println!("lx {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let config = load_config();

    let show_all = args.all || config.settings.all;
    let show_permissions = args.permissions || config.settings.permissions;
    let show_dirs = resolve_show_dirs(&args, &config.settings);

    let cwd = std::env::current_dir().unwrap();
    let input_path = cwd.join(&args.path);

    // Check if path exists and determine its type
    if input_path.exists() {
        // Path exists - check if it's a file or directory
        if input_path.is_dir() {
            // It's a directory - list contents
            list_directory(
                &input_path,
                None,
                show_all,
                show_permissions,
                show_dirs,
                &config.colors,
            );
        } else {
            // It's a file - show single file info
            show_single_file(&input_path, show_permissions, &config.colors);
        }
    } else if has_glob_chars(&args.path) {
        // Path doesn't exist but has glob chars - try as glob pattern
        let (target_dir, pattern) = parse_directory_and_pattern(&args.path);
        let full_target_dir = cwd.join(&target_dir);

        // If path exists, check if it's a directory to list
        if full_target_dir.exists() && full_target_dir.is_dir() {
            list_directory(
                &full_target_dir,
                pattern.as_ref(),
                show_all,
                show_permissions,
                show_dirs,
                &config.colors,
            );
        } else if pattern.is_some() {
            // Try to match files in current directory
            list_directory(
                &cwd,
                pattern.as_ref(),
                show_all,
                show_permissions,
                show_dirs,
                &config.colors,
            );
        } else {
            eprintln!(
                "lx: cannot access '{}': No such file or directory",
                args.path
            );
            std::process::exit(1);
        }
    } else {
        // Path doesn't exist and no glob chars - error
        eprintln!(
            "lx: cannot access '{}': No such file or directory",
            args.path
        );
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args_from(parts: &[&str]) -> Args {
        Args::try_parse_from(parts).unwrap()
    }

    #[test]
    fn no_showdir_hides_directories() {
        let args = args_from(&["lx", "--no-showdir"]);
        let settings = Settings {
            showdir: true,
            ..Settings::default()
        };

        assert!(!resolve_show_dirs(&args, &settings));
    }

    #[test]
    fn showdir_overrides_config_false() {
        let args = args_from(&["lx", "--showdir"]);
        let settings = Settings {
            showdir: false,
            ..Settings::default()
        };

        assert!(resolve_show_dirs(&args, &settings));
    }

    #[test]
    fn config_controls_showdir_without_cli_override() {
        let args = args_from(&["lx"]);
        let settings = Settings {
            showdir: false,
            ..Settings::default()
        };

        assert!(!resolve_show_dirs(&args, &settings));
    }

    #[test]
    fn permissions_identify_symlink_modes() {
        assert_eq!(get_permissions(0o120777), "lrwxrwxrwx");
    }

    #[test]
    fn sizes_are_human_readable() {
        assert_eq!(format_size(42, false), "42B");
        assert_eq!(format_size(1024, false), "1K");
        assert_eq!(format_size(1536, false), "1.5K");
        assert_eq!(format_size(0, true), "-");
    }
}
