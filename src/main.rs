use clap::Parser;
use std::fs;
use std::process::ExitCode;

// Use the library's public interface
use markdown_processor::{parse_markdown, process_actions, AppError, Summary};

// Modules defined within the binary crate
mod cli;
use cli::args::Cli; // Import the argument parser struct
use cli::output::print_summary; // Import the summary printing function

// --- Main Execution Logic ---

/// Orchestrates the entire process: parsing args, reading files, calling library, printing summary.
fn run() -> Result<Summary, AppError> {
    let cli = Cli::parse(); // Now the Parser trait is in scope, so parse() is found

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
    // Call the imported print_summary function
    print_summary(&summary, &resolved_output_dir_display);

    Ok(summary)
}

// --- Entry Point ---

/// Main application entry point. Calls `run` and handles errors.
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
