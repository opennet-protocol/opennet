pub struct QuicConfig {
    pub max_idle_timeout_ms: u64,
    pub max_concurrent_streams: u32,
}

impl Default for QuicConfig {
    fn default() -> Self {
        Self { max_idle_timeout_ms: 30000, max_concurrent_streams: 100 }
    }
}
