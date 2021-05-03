use super::traits::BuddiesStore;
use crate::lib::types::Buddy;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In memory storage for testing
#[derive(Clone)]
pub struct MemoryBuddiesStore {
    /// Represents a "buddies" table
    buddy_storage: Arc<RwLock<HashMap<Uuid, Buddy>>>,
}

impl MemoryBuddiesStore {
    pub fn new() -> Self {
        Self {
            buddy_storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl BuddiesStore for MemoryBuddiesStore {
    fn create_buddy(&mut self, buddy: Buddy) -> Result<()> {
        self.buddy_storage
            .write()
            .unwrap()
            .insert(buddy.id.clone(), buddy.clone());
        Ok(())
    }
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>> {
        let mut users_buddies = HashMap::new();
        let storage = self.buddy_storage.read().unwrap();
        for buddy in storage.values() {
            if buddy.user_id == user_id {
                users_buddies.insert(buddy.id.clone(), buddy.clone());
            }
        }
        Ok(users_buddies)
    }
}
