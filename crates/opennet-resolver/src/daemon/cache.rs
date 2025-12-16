use opennet_core::ServiceId;
use crate::candidates::CandidateSet;
use std::collections::BTreeMap;

pub struct ResolutionCache {
    entries: BTreeMap<ServiceId, CacheEntry>,
    ttl_secs: u64,
}

struct CacheEntry { candidates: CandidateSet, expires: u64 }

impl ResolutionCache {
    pub fn new(ttl_secs: u64) -> Self { Self { entries: BTreeMap::new(), ttl_secs } }
    pub fn get(&self, _id: &ServiceId) -> Option<&CandidateSet> { None }
    pub fn insert(&mut self, id: ServiceId, candidates: CandidateSet, now: u64) {
        self.entries.insert(id, CacheEntry { candidates, expires: now + self.ttl_secs });
    }
}

impl Default for ResolutionCache {
    fn default() -> Self { Self::new(300) }
}
