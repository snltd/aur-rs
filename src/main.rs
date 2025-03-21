use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
mod commands;
mod test_utils;
mod utils;

#[derive(Parser)]
#[clap(version, about = "Seriously opinionated FLAC and MP3 tagger", long_about = None)]
struct Cli {
    /// Be verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,
    /// Be silent
    #[arg(short, long, global = true)]
    pub quiet: bool,
    /// Say what would happen, without actually doing it
    #[arg(short, long, global = true)]
    pub noop: bool,
    /// Path to config file
    #[arg(short, long, global = true, default_value_t = utils::config::default_location())]
    pub config: Utf8PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// If the given files are in a disc_n directory, add (Disc n) to the album tag
    Albumdisc {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Renames and resizes artwork in the given directories
    Artfix {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Link non-square files to this directory for further processing.
        #[arg(short = 'd', long, global = true, default_value_t = utils::config::default_linkdir())]
        linkdir: Utf8PathBuf,
        /// Directories to proces
        #[arg(required = true)]
        directories: Vec<Utf8PathBuf>,
    },
    /// Re-encodes "hi-res" FLACs at CD quality
    Cdq {
        /// Leave the original files. New files will have -cdq before their suffix
        #[arg(short, long)]
        leave: bool,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
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
        files: Vec<Utf8PathBuf>,
    },
    /// Finds files in tracks/ which could be duplicates of tracks in albums/ or eps/
    Dupes { root_dir: Utf8PathBuf },
    /// Convert one or more FLACs to MP3s
    Flac2mp3 {
        /// Minimum MP3 bitrate
        #[arg(short, long, default_value = "128")]
        bitrate: String,
        /// Overwrite existing files
        #[arg(short, long)]
        force: bool,
        /// One or more FLAC files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Display a given property for the given file(s)
    Get {
        /// Don't show the filename
        #[arg(short, long)]
        short: bool,
        /// Property, e.g. time, title, or bitrate
        property: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Shows tag, time, and bitrate information about the given file(s)
    Info {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// For each given file, interactively a value for the given tag. Changes tag only
    Itag {
        /// The tag to modify
        tag: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Compares the given file(s) with our standards
    Lint {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Files and/or directories to check
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Compares the given file(s) with our standards
    Lintdir {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Directories to check
        #[arg(required = true)]
        directories: Vec<Utf8PathBuf>,
    },
    /// Shows tag information about files in the given directory, one file per line
    Ls {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Directories to list
        directories: Vec<Utf8PathBuf>,
    },
    /// Transcode a FLAC directory an equivalent point in the MP3 hierarchy
    Mp3dir {
        /// Minimum MP3 bitrate
        #[arg(short, long, default_value = "128")]
        bitrate: String,
        /// Suffix on the new directory name with the bitrate
        #[arg(short = 'x', long)]
        suffix: bool,
        /// Overwrite existing files
        #[arg(short, long)]
        force: bool,
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Root directory for media files, containing flac/ and mp3/
        #[arg(short = 'R', long, default_value = "/storage")]
        root: Utf8PathBuf,
        /// One or more directories
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// If the file name begins with a number, set its track number tag to that number
    Name2num {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Tags a file based on its name
    Name2tag {
        /// Ignore warnings, making a best effort
        #[arg(short, long)]
        force: bool,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Look for artists with similar, but not identical, names
    Namecheck { root_dir: Utf8PathBuf },
    /// Prefix the file's name with its zero-padded track number
    Num2name {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Uses ffmpeg to reencode files
    Reencode {
        /// Keep the original files after reencoding
        #[arg(short, long)]
        keep_originals: bool,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
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
        files: Vec<Utf8PathBuf>,
    },
    /// Correct capitalization across all tags
    Retitle {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Set a tag in one or more files
    Set {
        /// Tag name
        tag: String,
        /// Tag value
        value: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Put files into directories derived from their tags
    Sort {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Split a FLAC according to a .cue file with the same filename stem
    Split {
        /// One or more FLAC files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Remove embedded images and unwanted tags from the given file(s)
    Strip {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Ensure we have an MP3 for every FLAC. Assumes parallel flac/ and mp3/ trees
    Syncflac {
        /// Minimum MP3 bitrate
        #[arg(short, long, default_value = "128")]
        bitrate: String,
        /// Root directory for media files, containing flac/ and mp3/
        #[arg(short = 'R', long, default_value = "/storage")]
        root: Utf8PathBuf,
    },
    /// Rename the file(s) according to its tags
    Tag2name {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Display the raw tags for the given file(s)
    Tags {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
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
        files: Vec<Utf8PathBuf>,
    },
    /// Prefixes the artist name with "The" for all given file(s)
    Thes {
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Transcode one or more files with ffmpeg
    Transcode {
        ///  If that target exists, overwrite it
        #[arg(short, long)]
        force: bool,
        /// Remove the original files after transcoding
        #[arg(short = 'R', long = "remove")]
        remove_originals: bool,
        /// The desired format
        format: String,
        /// One or more media files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Checks media files are valid and uncorrupted
    Verify {
        /// Recurse
        #[arg(short, long)]
        recurse: bool,
        /// Files and/or directories to check
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Lists albums and EPs, or tracks, which exists as MP3 but not as FLAC
    Wantflac {
        /// Root directory for media files, containing flac/ and mp3/
        #[arg(short = 'R', long, default_value = "/storage")]
        root: Utf8PathBuf,
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
        quiet: cli.quiet,
        config: cli.config,
    };
    let result = match cli.command {
        Commands::Albumdisc { files } => commands::albumdisc::run(&files, &global_opts),
        Commands::Artfix {
            recurse,
            linkdir,
            directories,
        } => commands::artfix::run(&directories, recurse, linkdir, &global_opts),
        Commands::Cdq { files, leave } => commands::cdq::run(&files, leave, &global_opts),
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
        Commands::Flac2mp3 {
            bitrate,
            files,
            force,
        } => commands::flac2mp3::run(&files, bitrate, force, &global_opts),
        Commands::Get {
            property,
            files,
            short,
        } => commands::get::run(&property, &files, short),
        Commands::Info { files } => commands::info::run(&files),
        Commands::Itag { files, tag } => commands::itag::run(&files, &tag, &global_opts),
        Commands::Lint { recurse, files } => commands::lint::run(&files, recurse, &global_opts),
        Commands::Lintdir {
            recurse,
            directories,
        } => commands::lintdir::run(&directories, recurse, &global_opts),
        Commands::Ls {
            recurse,
            directories,
        } => commands::ls::run(&directories, recurse),
        Commands::Mp3dir {
            bitrate,
            files,
            force,
            recurse,
            root,
            suffix,
        } => commands::mp3dir::run(
            &files,
            &utils::types::Mp3dirOpts {
                bitrate,
                force,
                recurse,
                root,
                suffix,
            },
            &global_opts,
        ),
        Commands::Name2num { files } => commands::name2num::run(&files, &global_opts),
        Commands::Name2tag { files, force } => commands::name2tag::run(&files, force, &global_opts),
        Commands::Namecheck { root_dir } => commands::namecheck::run(&root_dir, &global_opts),
        Commands::Num2name { files } => commands::num2name::run(&files, &global_opts),
        Commands::Reencode {
            files,
            keep_originals,
        } => commands::reencode::run(&files, keep_originals),
        Commands::Renumber {
            direction,
            delta,
            files,
        } => commands::renumber::run(&direction, delta, &files, &global_opts),
        Commands::Retitle { files } => commands::retitle::run(&files, &global_opts),
        Commands::Set { tag, value, files } => {
            commands::set::run(&tag, &value, &files, &global_opts)
        }
        Commands::Sort { files } => commands::sort::run(&files, &global_opts),
        Commands::Split { files } => commands::split::run(&files),
        Commands::Strip { files } => commands::strip::run(&files),
        Commands::Syncflac { bitrate, root } => {
            commands::syncflac::run(&root, &bitrate, &global_opts)
        }
        Commands::Tagsub {
            tag,
            find,
            replace,
            files,
        } => commands::tagsub::run(&files, &tag, &find, &replace, &global_opts),
        Commands::Tag2name { files } => commands::tag2name::run(&files, &global_opts),
        Commands::Tags { files } => commands::tags::run(&files),
        Commands::Thes { files } => commands::thes::run(&files, &global_opts),
        Commands::Transcode {
            format,
            force,
            remove_originals,
            files,
        } => commands::transcode::run(
            &files,
            &format,
            &utils::types::TranscodeOptions {
                remove_originals,
                force,
            },
            &global_opts,
        ),
        Commands::Verify { recurse, files } => commands::verify::run(&files, recurse, &global_opts),
        Commands::Wantflac { root, tracks } => commands::wantflac::run(&root, tracks, &global_opts),
    };

    match result {
        Ok(result) => match result {
            true => std::process::exit(0),
            false => std::process::exit(2),
        },
        Err(e) => {
            handle_error(e);
            std::process::exit(1);
        }
    }
}
