//! Configuration loading.

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub node: NodeConfig,
    pub network: NetworkConfig,
    pub trust: TrustConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub data_dir: String,
    pub identity_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub bootstrap_peers: Vec<String>,
    pub max_peers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustConfig {
    pub warn_threshold: f64,
    pub critical_threshold: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node: NodeConfig {
                data_dir: "./opennet-data".to_string(),
                identity_file: "identity.key".to_string(),
            },
            network: NetworkConfig {
                listen_addr: "0.0.0.0:9000".to_string(),
                bootstrap_peers: Vec::new(),
                max_peers: 50,
            },
            trust: TrustConfig {
                warn_threshold: 0.15,
                critical_threshold: 0.05,
            },
        }
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    if path.as_ref().exists() {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}
