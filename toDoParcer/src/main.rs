//! Provides commands to parse `.todo` files or strings, show parse trees, and print credits.

use clap::{Parser, Subcommand};
use pest::Parser as PestParser;
use std::fs;
use to_do_parcer::parser::{ParseError, ToDoParser};

/// Defines CLI root arguments and subcommands.
#[derive(Parser)]
#[command(
    name = "to_do_parser",
    version = "1.0",
    about = "CLI tool for parsing ToDo files"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Supported CLI subcommands.
#[derive(Subcommand)]
enum Commands {
    Credits,
    Parse(ParseArgs),
}

/// Arguments for the `parse` subcommand.
#[derive(Parser)]
struct ParseArgs {
    #[arg(short, long)]
    file: String,

    #[arg(long)]
    tree: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Credits => {
            println!("Author: Tetiana Khaimyk");
            println!("Project: ToDo Parser");
            println!("Language: Rust");
        }

        Commands::Parse(args) => {
            if let Err(e) = run_parse(args) {
                eprintln!("Parsing error: {}", e);
            }
        }
    }
}

/// Handles the `parse` command.
///
/// # Arguments
/// * `args` — CLI arguments with file path and tree flag.
///
/// # Returns
/// * `Ok(())` if parsed successfully.
/// * `Err(ParseError)` if parsing fails.
fn run_parse(args: ParseArgs) -> Result<(), ParseError> {
    let content = fs::read_to_string(&args.file)?;

    if args.tree {
        let pairs = ToDoParser::parse(to_do_parcer::parser::Rule::file, &content)
            .map_err(|e| ParseError::Pest(Box::new(e)))?;
        println!("Syntax tree:\n");
        to_do_parcer::parser::display_tree(pairs);
    } else {
        let projects = ToDoParser::parse_projects(&content)?;
        for project in projects {
            project.display();
            println!();
        }
    }

    Ok(())
}
