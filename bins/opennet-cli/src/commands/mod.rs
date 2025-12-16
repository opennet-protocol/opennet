//! CLI commands.

pub mod resolve;
pub mod identity;
pub mod trust;
pub mod peer;
pub mod debug;

use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Resolve an open:// URI.
    Resolve {
        /// URI to resolve.
        uri: String,
    },
    /// Identity management.
    Identity {
        #[command(subcommand)]
        action: identity::IdentityAction,
    },
    /// Trust inspection.
    Trust {
        #[command(subcommand)]
        action: trust::TrustAction,
    },
    /// Peer management.
    Peer {
        #[command(subcommand)]
        action: peer::PeerAction,
    },
    /// Debug utilities.
    Debug {
        #[command(subcommand)]
        action: debug::DebugAction,
    },
}

pub async fn execute(cmd: Command) -> Result<()> {
    match cmd {
        Command::Resolve { uri } => resolve::run(&uri).await,
        Command::Identity { action } => identity::run(action).await,
        Command::Trust { action } => trust::run(action).await,
        Command::Peer { action } => peer::run(action).await,
        Command::Debug { action } => debug::run(action).await,
    }
}
