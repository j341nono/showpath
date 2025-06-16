use std::env;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("showpath")
        .version("1.0")
        .about("A handy tool for managing your Zsh environment")
        .arg(
            Arg::new("duplicates")
                .long("duplicates")
                .short('d')
                .help("Show duplicate entries in the PATH")
                .action(clap::ArgAction::SetTrue) // フラグとして設定
        )
        .arg(
            Arg::new("env")
                .long("env")
                .short('e')
                .num_args(0..=1)
                .value_name("VAR")
                .help("Display environment variables"),
        )
        .arg(
            Arg::new("zshrc")
                .long("zshrc")
                .short('z')
                .help("Show the contents of your .zshrc")
                .action(clap::ArgAction::SetTrue) // フラグとして設定
        )
        .arg(
            Arg::new("zshrc-search")
                .long("zshrc-search")
                .short('s')
                .num_args(1)
                .value_name("KEYWORD")
                .help("Search for a keyword in your .zshrc"),
        )
        .arg(
            Arg::new("zshrc-backup")
                .long("zshrc-backup")
                .short('b')
                .help("Create a backup of your .zshrc")
                .action(clap::ArgAction::SetTrue) // フラグとして設定
        )
        .get_matches();

    // PATH表示（デフォルト動作）
    if !matches.args_present() {
        show_path();
        return;
    }

    // 重複PATH検出
    if matches.get_flag("duplicates") {
        show_path_duplicates();
    }

    // 環境変数表示
    if matches.contains_id("env") {
        match matches.get_one::<String>("env") {
            Some(var_name) => {
                match env::var(var_name) {
                    Ok(val) => println!("{}={}", var_name, val),
                    Err(_) => println!("Environment variable '{}' does not exist.", var_name),
                }
            }
            None => {
                println!("🌍 Environment variables:");
                let mut vars: Vec<_> = env::vars().collect();
                vars.sort_by(|a, b| a.0.cmp(&b.0));
                for (k, v) in vars {
                    println!("{}={}", k, v);
                }
            }
        }
    }

    // .zshrc表示
    if matches.get_flag("zshrc") {
        println!("📄 ~/.zshrc contents:");
        print_file_with_lines(get_zshrc_path());
    }

    // .zshrc検索
    if let Some(keyword) = matches.get_one::<String>("zshrc-search") {
        println!("🔍 Searching for '{}' in ~/.zshrc:", keyword);
        search_file(get_zshrc_path(), keyword);
    }

    // .zshrcバックアップ
    if matches.get_flag("zshrc-backup") {
        backup_file(get_zshrc_path());
    }
}

// デフォルトのPATH表示
fn show_path() {
    println!("📁 Current PATH entries:");
    if let Ok(path) = env::var("PATH") {
        for (i, entry) in path.split(':').enumerate() {
            println!("{:>3}. {}", i + 1, entry);
        }
    } else {
        println!("❌ PATH environment variable not found.");
    }
}

// 重複PATHの検出
fn show_path_duplicates() {
    println!("🔍 Checking for duplicate PATH entries:");
    
    if let Ok(path) = env::var("PATH") {
        let entries: Vec<&str> = path.split(':').collect();
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = std::collections::HashSet::new();
        let mut found_duplicates = false;

        // 重複を検出
        for entry in &entries {
            if !seen.insert(entry) {
                duplicates.insert(entry);
            }
        }

        // 結果を表示
        for (i, entry) in entries.iter().enumerate() {
            if duplicates.contains(entry) {
                println!("{:>3}. 🔁 {} (duplicate)", i + 1, entry);
                found_duplicates = true;
            } else {
                println!("{:>3}.    {}", i + 1, entry);
            }
        }

        if !found_duplicates {
            println!("✅ No duplicate PATH entries found.");
        } else {
            println!("\n⚠️  Found {} duplicate entries", duplicates.len());
        }
    } else {
        println!("❌ PATH environment variable not found.");
    }
}

// .zshrcのパスを取得
fn get_zshrc_path() -> PathBuf {
    match env::var("HOME") {
        Ok(home) => PathBuf::from(home).join(".zshrc"),
        Err(_) => {
            eprintln!("❌ HOME environment variable not found.");
            std::process::exit(1);
        }
    }
}

// 行番号付き表示
fn print_file_with_lines(path: PathBuf) {
    match fs::read_to_string(&path) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().collect();
            if lines.is_empty() {
                println!("📄 File is empty.");
            } else {
                for (i, line) in lines.iter().enumerate() {
                    println!("{:>3}: {}", i + 1, line);
                }
                println!("\n📊 Total lines: {}", lines.len());
            }
        }
        Err(e) => {
            println!("❌ Failed to read file '{}': {}", path.display(), e);
        }
    }
}

// .zshrc検索
fn search_file(path: PathBuf, keyword: &str) {
    match fs::read_to_string(&path) {
        Ok(content) => {
            let mut matches_found = 0;
            
            for (i, line) in content.lines().enumerate() {
                if line.contains(keyword) {
                    println!("{:>3}: {}", i + 1, line);
                    matches_found += 1;
                }
            }
            
            if matches_found == 0 {
                println!("❌ No matches found for '{}'", keyword);
            } else {
                println!("\n📊 Found {} match(es)", matches_found);
            }
        }
        Err(e) => {
            println!("❌ Failed to read file '{}': {}", path.display(), e);
        }
    }
}

// バックアップ
fn backup_file(path: PathBuf) {
    if !path.exists() {
        println!("❌ File '{}' does not exist.", path.display());
        return;
    }

    let now = Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = path.with_extension(format!("bak.{}", now));
    
    match fs::copy(&path, &backup_path) {
        Ok(_) => {
            println!("✅ Backup created successfully!");
            println!("   Original: {}", path.display());
            println!("   Backup:   {}", backup_path.display());
            
            // ファイルサイズも表示
            if let Ok(metadata) = fs::metadata(&backup_path) {
                println!("   Size:     {} bytes", metadata.len());
            }
        }
        Err(e) => {
            println!("❌ Backup failed: {}", e);
        }
    }
}