//! Defines the command-line arguments structure.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,
    after_help = "Processes a structured markdown file to generate or delete files.\n\
                  Recognizes various header formats (see README/docs)."
)]
/// Holds the parsed command-line arguments.
pub struct Cli {
    /// Path to the markdown file containing the project structure.
    pub markdown_file: PathBuf,

    /// The base directory to create/delete files in (default: ./project-generated).
    #[arg(short, long, value_name = "DIR", default_value = "./project-generated")]
    pub output_dir: PathBuf,

    /// Overwrite existing files for 'File' actions.
    #[arg(short, long)]
    pub force: bool,
}
