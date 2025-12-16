pub struct DeterministicReplay {
    events: Vec<Vec<u8>>,
}

impl DeterministicReplay {
    pub fn new() -> Self { Self { events: Vec::new() } }
    pub fn record(&mut self, event: Vec<u8>) { self.events.push(event); }
    pub fn replay(&self) -> Vec<Vec<u8>> { self.events.clone() }
}

impl Default for DeterministicReplay {
    fn default() -> Self { Self::new() }
}
