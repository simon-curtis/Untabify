use clap::{arg, command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "An absolute file path or glob, example: \"*.sql\"")]
    pub file_path: String,

    #[arg(
        short,
        long,
        help = "[Optional] The number of spaces to use for each tab"
    )]
    pub tab_size: Option<usize>,

    #[arg(short, long, help = "The directory path of the directory to untabify")]
    pub dir: Option<String>,

    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    #[command(about = "Manage configuration")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Clone)]
pub enum ConfigAction {
    #[command(about = "Print the path of the config file")]
    Path,

    #[command(about = "Prints the config to the console")]
    Print,

    #[command(about = "Reset the config back to default settings")]
    Reset,

    #[command(about = "Set tab length preference")]
    Set {
        #[command(subcommand)]
        action: SetAction,
    },
}

#[derive(Subcommand, Clone)]
pub enum SetAction {
    TabSize {
        tab_size: usize,

        #[arg(short, long, help = "The extension to configure")]
        extension: Option<String>,
    },
}