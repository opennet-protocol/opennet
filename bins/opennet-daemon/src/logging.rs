//! Logging setup.

use crate::cli::Args;
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(args: &Args) -> Result<()> {
    let filter = if args.verbose {
        "debug"
    } else {
        "info"
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(filter))
        .init();

    Ok(())
}
