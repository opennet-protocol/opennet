//! Trust commands.

use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand, Debug)]
pub enum TrustAction {
    /// Inspect trust for a node.
    Inspect {
        /// Node ID (hex).
        node_id: String,
    },
    /// List trusted peers.
    List,
    /// Show trust graph stats.
    Stats,
}

pub async fn run(action: TrustAction) -> Result<()> {
    match action {
        TrustAction::Inspect { node_id } => {
            println!("Trust for node {}:", &node_id[..16]);
            println!("  Weight: 0.75");
            println!("  Epoch: 5");
        }
        TrustAction::List => {
            println!("Trusted peers:");
            println!("  (list would be shown)");
        }
        TrustAction::Stats => {
            println!("Trust graph statistics:");
            println!("  Nodes: 0");
            println!("  Edges: 0");
        }
    }
    Ok(())
}
