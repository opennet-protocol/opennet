//! Identity commands.

use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand, Debug)]
pub enum IdentityAction {
    /// Show current identity.
    Show,
    /// Generate new identity.
    Generate,
    /// Rotate keys.
    Rotate,
    /// Export public key.
    Export,
}

pub async fn run(action: IdentityAction) -> Result<()> {
    match action {
        IdentityAction::Show => {
            println!("Current identity information:");
            println!("  NodeId: (would be shown)");
            println!("  Epoch: 1");
        }
        IdentityAction::Generate => {
            println!("Generating new identity...");
        }
        IdentityAction::Rotate => {
            println!("Rotating keys...");
        }
        IdentityAction::Export => {
            println!("Public key: (would be exported)");
        }
    }
    Ok(())
}
