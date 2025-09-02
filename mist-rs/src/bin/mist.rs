use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mist", about = "Mist CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available installers
    List,
    /// Download an installer by ID to a destination path
    Download {
        /// Identifier of the installer
        id: usize,
        /// Destination file path
        dest: PathBuf,
    },
    /// Check for and apply updates to the Mist CLI
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            for inst in mist_core::installers::list_installers() {
                println!("{}: {}", inst.id, inst.name);
            }
        }
        Commands::Download { id, dest } => {
            mist_core::installers::download_installer(id, &dest).await?;
            println!("Downloaded installer {id} to {}", dest.display());
        }
        Commands::Update => {
            mist_core::helpers::updater::check_for_updates()?;
        }
    }

    Ok(())
}
