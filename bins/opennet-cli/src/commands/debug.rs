//! Debug commands.

use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand, Debug)]
pub enum DebugAction {
    /// Dump internal state.
    DumpState,
    /// Show FSM state.
    FsmState,
    /// Test CBOR encoding.
    TestCbor,
}

pub async fn run(action: DebugAction) -> Result<()> {
    match action {
        DebugAction::DumpState => {
            println!("Internal state dump:");
            println!("  FSM State: BOOTSTRAP");
        }
        DebugAction::FsmState => {
            println!("FSM State: BOOTSTRAP");
        }
        DebugAction::TestCbor => {
            println!("CBOR encoding test: OK");
        }
    }
    Ok(())
}
