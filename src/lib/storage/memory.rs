use super::traits::BuddiesStore;

#[derive(Clone)]
pub struct MemoryBuddiesStore {}

impl MemoryBuddiesStore {
    pub fn new() -> Self {
	Self {

	}
    }
}

impl BuddiesStore for MemoryBuddiesStore {}
