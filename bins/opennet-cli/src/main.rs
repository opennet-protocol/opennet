//! # OpenNet CLI
//!
//! Command-line tools for OpenNet management and debugging.
//!
//! ## Usage
//!
//! ```bash
//! # Resolve an open:// URI
//! opennet resolve open://chat.opennet
//!
//! # Show identity info
//! opennet identity show
//!
//! # Inspect trust graph
//! opennet trust inspect <node_id>
//!
//! # List peers
//! opennet peer list
//! ```

use anyhow::Result;
use clap::Parser;

mod commands;
mod output;

#[derive(Parser)]
#[command(name = "opennet")]
#[command(about = "OpenNet CLI tools")]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::execute(cli.command).await
}
