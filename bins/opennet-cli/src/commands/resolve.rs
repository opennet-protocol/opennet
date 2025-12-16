//! Resolve command.

use anyhow::Result;
use opennet_core::OpenNetUri;

pub async fn run(uri: &str) -> Result<()> {
    let parsed = OpenNetUri::parse(uri)?;
    
    println!("Resolving: {}", parsed);
    println!("  Domain: {}", parsed.domain);
    println!("  Path: {}", parsed.path);
    println!("  Scope: {}", parsed.scope);
    println!("  ServiceId: {}", parsed.service_id());

    // TODO: Actually resolve via daemon
    println!("\nResolution would be performed via daemon IPC.");

    Ok(())
}
