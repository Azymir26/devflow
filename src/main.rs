mod commands;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "devflow",
    version,
    about = "Manage your Git projects faster",
    long_about = "A CLI tool that streamlines common Git workflows:\n\n  \
        • Scaffold new projects with git init and a clean structure\n  \
        • View the status of every Git repo in a directory at a glance\n  \
        • Stage, commit, and push in a single command"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project with git init and basic structure
    New {
        /// Name of the project to create
        name: String,
    },
    /// Show git status of all projects in the current directory
    Status,
    /// Stage all changes, commit, and push in one step
    Push {
        /// Commit message
        message: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name } => commands::new_project(&name),
        Commands::Status => commands::status(),
        Commands::Push { message } => commands::push(&message),
    };

    if let Err(err) = result {
        eprintln!("{} {}", "error:".red().bold(), err);
        std::process::exit(1);
    }
}
