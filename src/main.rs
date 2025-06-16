use std::env;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("showpath")
        .version("1.0")
        .about("A handy tool for managing your Zsh environment")
        .arg(Arg::new("duplicates").long("duplicates").help("Show duplicate entries in the PATH"))
        .arg(
            Arg::new("env")
                .long("env")
                .num_args(0..=1)
                .value_name("VAR")
                .help("Display environment variables"),
        )
        .arg(Arg::new("zshrc").long("zshrc").help("Show the contents of your .zshrc"))
        .arg(
            Arg::new("zshrc-search")
                .long("zshrc-search")
                .num_args(1)
                .value_name("KEYWORD")
                .help("Search for a keyword in your .zshrc"),
        )
        .arg(Arg::new("zshrc-backup").long("zshrc-backup").help("Create a backup of your .zshrc"))
        .get_matches();

    // PATH表示
    if matches.args_present() == false {
        if let Ok(path) = env::var("PATH") {
            for entry in path.split(':') {
                println!("{}", entry);
            }
        }
        return;
    }

    if matches.contains_id("duplicates") {
        show_path_duplicates();
    }

    if matches.contains_id("env") {
        match matches.get_one::<String>("env") {
            Some(var_name) => {
                match env::var(var_name) {
                    Ok(val) => println!("{}={}", var_name, val),
                    Err(_) => println!("Environment variable {} does not exist.", var_name),
                }
            }
            None => {
                for (k, v) in env::vars() {
                    println!("{}={}", k, v);
                }
            }
        }
    }

    if matches.contains_id("zshrc") {
        print_file_with_lines(PathBuf::from(env::var("HOME").unwrap()).join(".zshrc"));
    }

    if let Some(keyword) = matches.get_one::<String>("zshrc-search") {
        search_file(PathBuf::from(env::var("HOME").unwrap()).join(".zshrc"), keyword);
    }

    if matches.contains_id("zshrc-backup") {
        backup_file(PathBuf::from(env::var("HOME").unwrap()).join(".zshrc"));
    }
}

// 重複PATHの検出
fn show_path_duplicates() {
    if let Ok(path) = env::var("PATH") {
        let mut seen = std::collections::HashSet::new();
        for entry in path.split(':') {
            if !seen.insert(entry) {
                println!("🔁 Duplicate: {}", entry);
            } else {
                println!("{}", entry);
            }
        }
    }
}

// 行番号付き表示
fn print_file_with_lines(path: PathBuf) {
    if let Ok(content) = fs::read_to_string(path) {
        for (i, line) in content.lines().enumerate() {
            println!("{:>3}: {}", i + 1, line);
        }
    } else {
        println!("File not found.");
    }
}

// .zshrc検索
fn search_file(path: PathBuf, keyword: &str) {
    if let Ok(content) = fs::read_to_string(path) {
        for (i, line) in content.lines().enumerate() {
            if line.contains(keyword) {
                println!("{:>3}: {}", i + 1, line);
            }
        }
    } else {
        println!("File not found.");
    }
}

// バックアップ
fn backup_file(path: PathBuf) {
    let now = Local::now().format("%Y%m%d");
    let backup_path = path.with_extension(format!("bak.{}", now));
    if let Err(e) = fs::copy(&path, &backup_path) {
        println!("Backup failed: {}", e);
    } else {
        println!("✅ Backup created: {}", backup_path.display());
    }
}
