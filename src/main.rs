mod commands;
mod types;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use commands::{cmd_find, cmd_list, cmd_search, cmd_estimate};
use utils::OutputFormat;

#[derive(Parser)]
#[command(name = "scout")]
#[command(about = "Fast file finder for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    root: Option<PathBuf>,

    #[arg(short = 'F', long, global = true, default_value = "toon")]
    format: String,

    #[arg(long, global = true)]
    stream: bool,
}

#[derive(Subcommand)]
enum Commands {
    Find {
        pattern: String,
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    Search {
        query: String,
        #[arg(short, long)]
        ext: Option<String>,
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },
    List {
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    Estimate {
        #[arg(short, long, default_value = "1000")]
        limit: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root.clone().unwrap_or(std::env::current_dir()?);

    let format = OutputFormat::from_str(&cli.format)
        .unwrap_or_else(|| {
            eprintln!("Invalid format '{}'. Valid options: toon, json, plain", cli.format);
            std::process::exit(1);
        });

    match cli.command {
        Commands::Find { pattern, limit } => {
            cmd_find(&root, &pattern, limit, format, cli.stream)?;
        }
        Commands::Search { query, ext, limit } => {
            cmd_search(&root, &query, ext.as_deref(), limit, format, cli.stream)?;
        }
        Commands::List { limit } => {
            cmd_list(&root, limit, format, cli.stream)?;
        }
        Commands::Estimate { limit } => {
            cmd_estimate(&root, limit)?;
        }
    }

    Ok(())
}
