use clap::{Parser, Subcommand};
use std::path::PathBuf;

use mist_rs::{clear_cache, download_firmware, export_installers, Format};

#[derive(Parser)]
#[command(name = "mist", about = "Mist CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download an installer by ID to a destination path
    Download {
        /// Identifier of the installer
        id: usize,
        /// Destination file path
        dest: PathBuf,
        /// Number of retries if the download fails
        #[arg(long, default_value_t = 3)]
        retries: usize,
        /// Delay in seconds between retries
        #[arg(long, default_value_t = 5)]
        delay: u64,
    },
    /// Export a list of available installers
    Export {
        /// Only include installers whose names contain this string
        #[arg(long)]
        filter: Option<String>,
        /// Output format
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// Optional file path to write output to
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Clear the installer cache
    Cache {
        /// Optional path to the cache directory
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Check for and apply updates to the Mist CLI
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download {
            id,
            dest,
            retries,
            delay,
        } => {
            download_firmware(id, &dest, retries, delay).await?;
            println!("Downloaded installer {id} to {}", dest.display());
        }
        Commands::Export {
            filter,
            format,
            output,
        } => {
            let data = export_installers(filter.as_deref(), format)?;
            if let Some(path) = output {
                std::fs::write(&path, data)?;
                println!("Wrote installer list to {}", path.display());
            } else {
                println!("{data}");
            }
        }
        Commands::Cache { path } => {
            clear_cache(path.as_deref())?;
            println!("Cache cleared");
        }
        Commands::Update => {
            mist_core::helpers::updater::check_for_updates()?;
        }
    }

    Ok(())
}
