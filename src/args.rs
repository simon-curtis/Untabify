use clap::{arg, command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    #[command(about = "Untabify a file")]
    File {
        #[arg(help = "The path of the file to untabify")]
        path: String,

        #[arg(
            short,
            long,
            default_value = "The configured tab size for the file extension",
            help = "The number of spaces to use for each tab"
        )]
        tab_size: Option<usize>,
    },

    #[command(about = "Untabify an entire directory")]
    Dir {
        #[arg(help = "The directory path of the directory to untabify")]
        path: String,

        #[arg(short, long, help = "The file predicate, example: \"*.sql\"")]
        glob: Option<String>,

        #[arg(
            short,
            long,
            default_value = "The configured tab size for the file extension",
            help = "The number of spaces to use for each tab"
        )]
        tab_size: Option<usize>,
    },

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
