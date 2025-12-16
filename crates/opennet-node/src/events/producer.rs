//! Event producer registry.

use crate::fsm::StateEvent;

/// Trait for components that produce FSM events.
pub trait EventProducer {
    /// Drain pending events.
    fn drain_events(&mut self) -> Vec<StateEvent>;
}

/// Registry of event producers.
pub struct ProducerRegistry {
    producers: Vec<Box<dyn EventProducer + Send>>,
}

impl ProducerRegistry {
    pub fn new() -> Self {
        Self { producers: Vec::new() }
    }

    pub fn register(&mut self, producer: Box<dyn EventProducer + Send>) {
        self.producers.push(producer);
    }

    pub fn collect_events(&mut self) -> Vec<StateEvent> {
        self.producers.iter_mut().flat_map(|p| p.drain_events()).collect()
    }
}

impl Default for ProducerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
