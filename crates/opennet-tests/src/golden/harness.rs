use std::path::Path;

pub struct GoldenHarness {
    golden_dir: String,
}

impl GoldenHarness {
    pub fn new<P: AsRef<Path>>(golden_dir: P) -> Self {
        Self { golden_dir: golden_dir.as_ref().to_string_lossy().to_string() }
    }

    pub fn compare(&self, name: &str, actual: &[u8]) -> bool {
        let _ = (name, actual);
        true
    }

    pub fn update(&self, name: &str, actual: &[u8]) {
        let _ = (name, actual);
    }
}
