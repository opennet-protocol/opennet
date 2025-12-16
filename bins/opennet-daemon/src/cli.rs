//! CLI argument parsing.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "opennet-daemon")]
#[command(about = "OpenNet full node daemon")]
pub struct Args {
    /// Configuration file path.
    #[arg(short, long, default_value = "opennet.toml")]
    pub config: PathBuf,

    /// Enable verbose logging.
    #[arg(short, long)]
    pub verbose: bool,

    /// Listen address.
    #[arg(long, default_value = "0.0.0.0:9000")]
    pub listen: String,

    /// Data directory.
    #[arg(long, default_value = "./opennet-data")]
    pub data_dir: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
