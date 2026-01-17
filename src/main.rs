mod commands;
mod filter;
mod types;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use commands::{cmd_find, cmd_list, cmd_search};
use filter::build_exclude_patterns;

#[derive(Parser)]
#[command(name = "scout")]
#[command(about = "Fast file finder for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    root: Option<PathBuf>,

    #[arg(long, global = true, default_value_t = true, action = clap::ArgAction::SetTrue)]
    #[arg(long = "no-gitignore", global = true, action = clap::ArgAction::SetFalse)]
    gitignore: bool,

    #[arg(long, global = true, default_value_t = true, action = clap::ArgAction::SetTrue)]
    #[arg(long = "no-hidden", global = true, action = clap::ArgAction::SetFalse)]
    hidden: bool,

    #[arg(long, global = true)]
    exclude: Option<String>,
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root.clone().unwrap_or(std::env::current_dir()?);
    let excludes = build_exclude_patterns(&cli.exclude);

    match cli.command {
        Commands::Find { pattern, limit } => {
            cmd_find(&root, &pattern, limit, cli.gitignore, cli.hidden, &excludes)?;
        }
        Commands::Search { query, ext, limit } => {
            cmd_search(&root, &query, ext.as_deref(), limit, cli.gitignore, cli.hidden, &excludes)?;
        }
        Commands::List { limit } => {
            cmd_list(&root, cli.gitignore, cli.hidden, &excludes, limit)?;
        }
    }

    Ok(())
}
