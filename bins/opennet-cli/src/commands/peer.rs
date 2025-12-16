//! Peer commands.

use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand, Debug)]
pub enum PeerAction {
    /// List connected peers.
    List,
    /// Connect to a peer.
    Connect {
        /// Peer address.
        addr: String,
    },
    /// Disconnect from a peer.
    Disconnect {
        /// Node ID (hex).
        node_id: String,
    },
}

pub async fn run(action: PeerAction) -> Result<()> {
    match action {
        PeerAction::List => {
            println!("Connected peers:");
            println!("  (none)");
        }
        PeerAction::Connect { addr } => {
            println!("Connecting to {}...", addr);
        }
        PeerAction::Disconnect { node_id } => {
            println!("Disconnecting from {}...", &node_id[..16]);
        }
    }
    Ok(())
}
