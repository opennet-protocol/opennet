use std::path::PathBuf;

/// Node configuration.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Data directory.
    pub data_dir: PathBuf,
    /// Listen address.
    pub listen_addr: String,
    /// Bootstrap peers.
    pub bootstrap_peers: Vec<String>,
    /// Trust thresholds.
    pub trust_warn_threshold: f64,
    pub trust_critical_threshold: f64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./opennet-data"),
            listen_addr: "0.0.0.0:9000".to_string(),
            bootstrap_peers: Vec::new(),
            trust_warn_threshold: 0.15,
            trust_critical_threshold: 0.05,
        }
    }
}
