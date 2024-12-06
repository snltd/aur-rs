use clap::{Parser, Subcommand};
mod commands;
use std::path::PathBuf;
mod utils;

#[derive(Parser)]
#[clap(version, about = "Seriously opinionated FLAC and MP3 tagger", long_about = None)]

struct Cli {
    /// Be verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,
    /// Say what would happen, without actually doing it (currently not implemented)
    #[arg(short, long, global = true)]
    pub noop: bool,
    /// Path to config file
    #[arg(short, long, global = true, default_value_t = utils::config::default_location())]
    pub config: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// If the given files are in a disc_n directory, add (Disc n) to the album tag
    Albumdisc {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Renames and resizes artwork in the given directories
    Artfix {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Link non-square files to this directory for further processing.
        #[arg(short = 'd', long, global = true, default_value_t = utils::config::default_linkdir())]
        linkdir: String,
        /// Directories to proces
        #[arg(required = true)]
        directories: Vec<String>,
    },
    /// Re-encodes "hi-res" FLACs at CD quality
    Cdq {
        /// Leave the original files. New files will have -cdq before their suffix
        #[arg(short, long)]
        leave: bool,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Assuming parallel flac/ and mp3/ directories, copies tags from FLACs to MP3s
    Copytags {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Force tag copy: otherwise tags are only copied if the source is newer
        #[arg(short, long)]
        force: bool,
        /// Files and/or directories to retag
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Finds files in tracks/ which could be duplicates of tracks in albums/ or eps/.
    Dupes { root_dir: String },
    /// Display a given property for the given file(s)
    Get {
        /// Property, e.g. time, title, or bitrate
        property: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Shows tag, time, and bitrate information about the given file(s)
    Info {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// For each given file, interactively supply a track number
    Inumber {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Compares the given file(s) with our standards
    Lint {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Files and/or directories to retag
        #[arg(required = true)]
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
    /// If the file name begins with a number, set its track number tag to that number
    Name2num {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Tags a file based on its name
    Name2tag {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Look for artists with similar, but not identical, names
    Namecheck { root_dir: String },
    /// Prefix the file's name with its zero-padded track number
    Num2name {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Increments or decrements tag and filename numbers
    Renumber {
        /// renumber up or down
        #[arg(value_enum)]
        direction: utils::types::RenumberDirection,
        /// Increment/decrement delta
        delta: u32,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Correct capitalization across all tags
    Retitle {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Set a tag in one or more files
    Set {
        /// Tag name
        tag: String,
        /// Tag value
        value: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Remove embedded images and unwanted tags from the given file(s)
    Strip {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Rename the file(s) according to its tags
    Tag2name {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Display the raw tags for the given file(s)
    Tags {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Globally find and replace in the given tag. Accepts any Rust regex
    Tagsub {
        /// Tag on which to operate
        tag: String,
        /// Find pattern
        find: String,
        /// Replace string
        replace: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Prefixes the artist name with "The" for all given file(s)
    Thes {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Lists albums and EPs, or tracks, which exists as MP3 but not as FLAC
    Wantflac {
        /// Root directory for media files, containing flac/ and mp3/
        #[arg(short = 'R', long, default_value = "/storage")]
        root: String,
        /// Find tracks rather than albums/eps
        #[arg(short = 'T', long)]
        tracks: bool,
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
    let global_opts = crate::utils::types::GlobalOpts {
        verbose: cli.verbose,
        noop: cli.noop,
        config: PathBuf::from(cli.config),
    };
    let result = match cli.command {
        Commands::Albumdisc { files } => commands::albumdisc::run(&files, &global_opts),
        Commands::Artfix {
            recurse,
            linkdir,
            directories,
        } => commands::artfix::run(&directories, recurse, linkdir),
        Commands::Cdq { files, leave } => commands::cdq::run(&files, leave),
        Commands::Copytags {
            recurse,
            force,
            files,
        } => commands::copytags::run(
            &files,
            &utils::types::CopytagsOptions { recurse, force },
            &global_opts,
        ),
        Commands::Dupes { root_dir } => commands::dupes::run(&root_dir),
        Commands::Get { property, files } => commands::get::run(&property, &files),
        Commands::Info { files } => commands::info::run(&files),
        Commands::Inumber { files } => commands::inumber::run(&files),
        Commands::Lint { recurse, files } => commands::lint::run(&files, recurse, &global_opts),
        Commands::Ls {
            recurse,
            directories,
        } => commands::ls::run(&directories, recurse),
        Commands::Name2num { files } => commands::name2num::run(&files, &global_opts),
        Commands::Name2tag { files } => commands::name2tag::run(&files, &global_opts),
        Commands::Namecheck { root_dir } => commands::namecheck::run(&root_dir, &global_opts),
        Commands::Num2name { files } => commands::num2name::run(&files),
        Commands::Renumber {
            direction,
            delta,
            files,
        } => commands::renumber::run(&direction, delta, &files),
        Commands::Retitle { files } => commands::retitle::run(&files, &global_opts),
        Commands::Set { tag, value, files } => commands::set::run(&tag, &value, &files),
        Commands::Strip { files } => commands::strip::run(&files),
        Commands::Tagsub {
            tag,
            find,
            replace,
            files,
        } => commands::tagsub::run(&files, &tag, &find, &replace, &global_opts),
        Commands::Tag2name { files } => commands::tag2name::run(&files),
        Commands::Tags { files } => commands::tags::run(&files),
        Commands::Thes { files } => commands::thes::run(&files),
        Commands::Wantflac { root, tracks } => commands::wantflac::run(&root, tracks, &global_opts),
    };

    if let Err(e) = result {
        handle_error(e);
        std::process::exit(1);
    }
}
