pub struct BackpressureController { window_size: u32, used: u32 }

impl BackpressureController {
    pub fn new(window_size: u32) -> Self { Self { window_size, used: 0 } }
    pub fn available(&self) -> u32 { self.window_size.saturating_sub(self.used) }
    pub fn consume(&mut self, amount: u32) { self.used = self.used.saturating_add(amount); }
    pub fn release(&mut self, amount: u32) { self.used = self.used.saturating_sub(amount); }
}

impl Default for BackpressureController {
    fn default() -> Self { Self::new(65536) }
}
