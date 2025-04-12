use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf}; // Added Path import
use std::process::ExitCode;

// Use the library's public interface
// REMOVED: print_summary import from library
use markdown_processor::{parse_markdown, process_actions, AppError, Summary};

// --- Argument Parsing ---

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,
    after_help = "Processes a structured markdown file to generate or delete files.\n\
                  Recognizes various header formats (see README/docs)."
)]
struct Cli {
    /// Path to the markdown file containing the project structure.
    markdown_file: PathBuf,

    /// The base directory to create/delete files in (default: ./project-generated).
    #[arg(short, long, value_name = "DIR", default_value = "./project-generated")]
    output_dir: PathBuf, // Use default_value directly

    /// Overwrite existing files for 'File' actions.
    #[arg(short, long)]
    force: bool,
}

// --- Summary Printing Logic (Moved from library) ---

/// Prints the final processing summary to the console.
/// This function now lives within the binary crate.
fn print_summary(summary: &Summary, resolved_base: &Path) {
    println!("{}", "-".repeat(40));
    println!("Processing Summary:");
    println!(
        "  Base Directory:                     {}",
        resolved_base.display()
    );
    println!("  Files created:                      {}", summary.created);
    println!(
        "  Files overwritten (--force):        {}",
        summary.overwritten
    );
    println!("  Files deleted:                      {}", summary.deleted);
    println!("{}", "-".repeat(14) + " Skipped " + &"-".repeat(19));
    println!(
        "  Skipped (create, exists):           {}",
        summary.skipped_exists
    );
    println!(
        "  Skipped (delete, not found):        {}",
        summary.skipped_not_found
    );
    println!(
        "  Skipped (delete, is dir):           {}",
        summary.skipped_isdir_delete
    );
    println!(
        "  Skipped (delete, other type):       {}",
        summary.skipped_other_type
    );
    println!("{}", "-".repeat(12) + " Failed/Errors " + &"-".repeat(13));
    println!(
        "  Failed (unsafe/invalid path):       {}",
        summary.failed_unsafe
    );
    println!(
        "  Failed (create, target is dir):     {}",
        summary.failed_isdir_create
    );
    println!(
        "  Failed (create, parent is file):    {}",
        summary.failed_parent_isdir
    );
    println!(
        "  Failed (I/O or Path error):         {}",
        summary.failed_io
    );
    println!(
        "  Failed (other unexpected errors):   {}",
        summary.error_other
    );
    println!("{}", "-".repeat(40));
}

// --- Main Execution Logic ---

fn run() -> Result<Summary, AppError> {
    let cli = Cli::parse();

    // Resolve markdown file path for clearer error messages
    let resolved_md_path = cli.markdown_file.canonicalize().map_err(AppError::Io)?;

    println!("Reading markdown file: {}", resolved_md_path.display());
    let markdown_content = fs::read_to_string(&resolved_md_path)?;

    println!("\nParsing markdown for file actions...");
    let parsed_actions = parse_markdown(&markdown_content)?; // Use lib function

    if parsed_actions.is_empty() {
        // Basic check if content might have had actionable items
        if markdown_content.contains("```")
            || markdown_content.contains("//")
            || markdown_content.contains("**")
            || markdown_content.contains("##")
        {
            eprintln!("\nWarning: No valid actions extracted. Check formatting.");
        } else {
            println!("\nInfo: No actionable content found.");
        }
        println!("\nNo actions to process.");
        return Ok(Summary::default()); // Return empty summary
    }

    println!(
        "\nFound {} actions to process (sorted by document order).",
        parsed_actions.len()
    );

    // Process actions using the library function
    let summary = process_actions(&cli.output_dir, parsed_actions, cli.force)?;

    // Print summary needs the *resolved* base path for display
    // Resolve again for printing; process_actions resolves internally for safety.
    // Use original path if canonicalize fails (e.g., dir deleted during processing).
    let resolved_output_dir_display = cli.output_dir.canonicalize().unwrap_or(cli.output_dir);
    // Call the local print_summary function defined above
    print_summary(&summary, &resolved_output_dir_display);

    Ok(summary)
}

// --- Entry Point ---

fn main() -> ExitCode {
    match run() {
        Ok(_) => {
            println!("\nProject file processing completed successfully.");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("\nError: {}", err);
            // Provide context based on the error type
            match err {
                AppError::Parse(p_err) => eprintln!("  Stage: Parsing\n  Details: {}", p_err),
                AppError::Process(pr_err) => {
                    eprintln!("  Stage: Processing\n  Details: {}", pr_err)
                }
                AppError::Io(io_err) => eprintln!("  Stage: File I/O\n  Details: {}", io_err),
                AppError::Argument(arg_err) => {
                    eprintln!("  Stage: Arguments\n  Details: {}", arg_err)
                }
            }
            ExitCode::FAILURE
        }
    }
}
