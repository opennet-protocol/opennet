use super::binding::SessionBinding;
use std::collections::BTreeMap;

pub struct SessionManager {
    sessions: BTreeMap<SessionBinding, u64>,
}

impl SessionManager {
    pub fn new() -> Self { Self { sessions: BTreeMap::new() } }
    pub fn register(&mut self, binding: SessionBinding, session_id: u64) {
        self.sessions.insert(binding, session_id);
    }
    pub fn remove(&mut self, binding: &SessionBinding) { self.sessions.remove(binding); }
    pub fn get(&self, binding: &SessionBinding) -> Option<u64> { self.sessions.get(binding).copied() }
}

impl Default for SessionManager {
    fn default() -> Self { Self::new() }
}
