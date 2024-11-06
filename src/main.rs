use clap::{arg, command, Parser, Subcommand};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // read in file from args
    let args: Args = Args::parse();
    match &args.cmd {
        Commands::File { path } => untabify_file(&path, &args.tab_size),
        Commands::Dir { path, glob } => untabify_dir(path, glob, &args.tab_size),
    }
}

fn untabify_file(path: &str, tab_size: &usize) {
    println!("Untabifying file: {}", path);

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

fn untabify_dir(dir_path: &str, glob: &Option<String>, tab_size: &usize) {
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
                _ => untabify_file(path, tab_size),
            };
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 4,
        help = "The number of spaces to use for each tab"
    )]
    tab_size: usize,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    #[command(about = "Untabify a file")]
    File {
        #[arg(help = "The path of the file to untabify")]
        path: String,
    },

    #[command(about = "Untabify an entire directory")]
    Dir {
        #[arg(help = "The directory path of the directory to untabify")]
        path: String,

        #[arg(short, long, help = "The file predicate, example: \"*.sql\"")]
        glob: Option<String>,
    },
}
