use args::{Args, Commands, ConfigAction, SetAction};
use clap::Parser;
use config::{Config, ConfigDefaults};
use std::path::Path;
use walkdir::WalkDir;

mod args;
mod config;

fn main() {
    let config_path = ConfigDefaults::config_path();
    let mut config = Config::load(&config_path);
    let args: Args = Args::parse();

    match &args.cmd {
        Commands::File { path, tab_size } => untabify_file(path, tab_size, &config),
        Commands::Dir {
            path,
            glob,
            tab_size,
        } => untabify_dir(path, glob, tab_size, &config),
        Commands::Config { action } => match action {
            ConfigAction::Print => println!("{}", serde_json::to_string_pretty(&config).unwrap()),
            ConfigAction::Path => {
                println!("Current path can be found:\n\n{}", config.config_path())
            }
            ConfigAction::Reset => {
                let config = Config::default();
                config.save();
                println!("Config has been reset to default values");
            }
            ConfigAction::Set { action } => match action {
                SetAction::TabSize {
                    tab_size,
                    extension,
                } => match extension {
                    Some(extension) => {
                        config.set_tab_size(tab_size, Some(extension));
                        println!("Set tab size for extension {} to {}", extension, tab_size);
                    }
                    None => {
                        config.set_tab_size(tab_size, None);
                        println!("Set default tab size to {}", tab_size);
                    }
                },
            },
        },
    }
}

fn untabify_file(path: &str, tab_size: &Option<usize>, config: &Config) {
    println!("Untabifying file: {}", path);

    let path = Path::new(path);
    let tab_size = match tab_size {
        Some(ts) => ts,
        None => {
            let extension = match path.extension() {
                Some(ext) => ext.to_str().unwrap().to_lowercase(),
                None => "default".to_string(),
            };
            config.get_tab_size(&extension)
        }
    };

    let converted = std::fs::read_to_string(path)
        .expect("failed to read file")
        .lines()
        .map(|line| {
            if line.is_empty() {
                return line.to_string();
            }

            let mut new_line = String::new();
            let mut i = 0;
            for c in line.chars() {
                match c {
                    '\t' => {
                        let spaces = tab_size - (i % tab_size);
                        new_line.push_str(&" ".repeat(spaces));
                        i += spaces;
                    }
                    _ => {
                        new_line.push(c);
                        i += 1;
                    }
                }
            }

            new_line
        })
        .collect::<Vec<String>>()
        .join("\n");

    std::fs::write(path, converted).expect("failed to write file");
}

fn untabify_dir(dir_path: &str, glob: &Option<String>, tab_size: &Option<usize>, config: &Config) {
    let path = Path::new(dir_path);
    if !path.is_dir() {
        println!("Path {} is not a directory", dir_path);
        return;
    }

    let glob = glob
        .as_ref()
        .map(|g| glob::Pattern::new(g).expect("Failed to parse glob"));

    println!("Untabifying directory: {}", dir_path);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry
                .path()
                .to_str()
                .expect("Failed to convert path to string");

            match glob {
                Some(ref glob) => {
                    if !glob.matches(path) {
                        continue;
                    }
                }
                _ => untabify_file(path, tab_size, &config),
            };
        }
    }
}
