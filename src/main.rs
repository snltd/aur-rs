use clap::{Parser, Subcommand};
mod commands;
mod common;

// use crate::common::info;
// use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, about = "Seriously opinionated FLAC and MP3 tagger", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Shows tag, time, and bitrate information about the given file(s)
    Info {
        /// One or more media files
        files: Vec<String>,
    },

    /// Display the raw tags for the given file(s)
    Tags {
        /// One or more media files
        files: Vec<String>,
    },
    /// Shows tag information about files in the given directory, one file per line
    Ls {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Directories to list
        directories: Vec<String>,
    },
    /// Display a given property for the given file(s)
    Get {
        /// Property, e.g. time, title, or bitrate
        property: String,
        /// One or more media files
        files: Vec<String>,
    },
}

fn handle_error(err: anyhow::Error) {
    if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
        eprintln!("ERROR: (I/O) : {}", io_err);
    } else if let Some(parse_err) = err.downcast_ref::<std::num::ParseIntError>() {
        eprintln!("ERROR: (Parsing): {}", parse_err);
    } else {
        eprintln!("ERROR: {}", err);
    }
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Get { property, files } => commands::get::run(&property, files),
        Commands::Info { files } => commands::info::run(files),
        Commands::Tags { files } => commands::tags::run(files),
        Commands::Ls {
            recurse,
            directories,
        } => commands::ls::run(directories, recurse),
    };

    if let Err(e) = result {
        handle_error(e);
        std::process::exit(1);
    }
}
