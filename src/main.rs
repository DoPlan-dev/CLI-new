use anyhow::Result;
use clap::{Parser, Subcommand};

// Re-export from lib for binary
use doplan::commands::{discuss, generate, plan, implement, next, progress, phase};

#[derive(Parser)]
#[command(name = "doplan")]
#[command(about = "Project workflow automation tool", long_about = None)]
#[command(version = "0.0.20-beta")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Launch TUI interface
    #[arg(long)]
    tui: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install DoPlan in your project
    Install,
    /// Show project dashboard
    Dashboard,
    /// Run development server
    Server,
    /// Sync GitHub data
    Github,
    /// Execute IDE command (internal use)
    #[command(hide = true)]
    Execute {
        /// Command name
        command: String,
        /// Command arguments
        args: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle TUI flag
    if cli.tui {
        // TODO: Launch TUI
        println!("TUI not yet implemented");
        return Ok(());
    }

    // Handle commands
    match cli.command {
        Some(Commands::Install) => {
            println!("Install command not yet implemented");
            Ok(())
        }
        Some(Commands::Dashboard) => {
            println!("Dashboard command not yet implemented");
            Ok(())
        }
        Some(Commands::Server) => {
            println!("Server command not yet implemented");
            Ok(())
        }
        Some(Commands::Github) => {
            println!("Github command not yet implemented");
            Ok(())
        }
        Some(Commands::Execute { command, args }) => {
            match command.as_str() {
                "discuss" => discuss::execute(args).await,
                "generate" => generate::execute(args).await,
                "plan" => plan::execute(args).await,
                "implement" => implement::execute(args).await,
                "next" => next::execute(args).await,
                "progress" => progress::execute(args).await,
                "phase" => phase::execute(args).await,
                _ => {
                    eprintln!("Unknown command: {}", command);
                    Ok(())
                }
            }
        }
        None => {
            // Show minimal TUI menu (context-aware)
            println!("TUI not yet implemented");
            Ok(())
        }
    }
}

