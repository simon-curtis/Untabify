use args::{Args, Command, ConfigAction, SetAction};
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
        Some(cmd) => match cmd {
            Command::Config { action } => match action {
                ConfigAction::Print => {
                    println!("{}", serde_json::to_string_pretty(&config).unwrap())
                }
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
        },
        None => untabify_files(&args, &config),
    }
}

fn untabify_files(args: &Args, config: &Config) {
    let wd = match args.dir {
        Some(ref dir_path) => {
            let wd = Path::new(dir_path);
            if !wd.is_dir() {
                println!("Path {} is not a directory", dir_path);
                return;
            }
            println!("Untabifying directory: {}", dir_path);
            wd
        }
        None => Path::new("."),
    };

    let path = Path::new(&args.file_path);

    if path.is_absolute() {
        if path.exists() {
            untabify_file(&path, &args.tab_size, config)
        }
        else {
            println!("File \"{}\" does not exist", path.display());
        }
    }
    else {
        let glob = glob::Pattern::new(&args.file_path).expect("Failed to parse glob");

        WalkDir::new(wd)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_owned())
            .filter(|path| glob.matches(path.to_str().unwrap()))
            .for_each(|path| untabify_file(&path, &args.tab_size, config));
    }
}

fn untabify_file(path: &Path, tab_size: &Option<usize>, config: &Config) {
    println!("Untabifying file: {}", path.display());

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

            new_line.trim_end().to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");

    std::fs::write(path, converted).expect("failed to write file");
}