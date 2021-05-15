use super::traits::BuddiesStore;
use crate::lib::types::{Buddy, Interaction, Timestamp};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use uuid::Uuid;

/// In memory storage for testing
#[derive(Clone)]
pub struct MemoryBuddiesStore {
    /// Represents a "buddies" table
    buddy_storage: Arc<RwLock<HashMap<Uuid, Buddy>>>,
    /// Represents an "interactions" table
    interaction_storage: Arc<RwLock<HashMap<Uuid, Interaction>>>,
}

impl MemoryBuddiesStore {
    pub fn new() -> Self {
        Self {
            buddy_storage: Arc::new(RwLock::new(HashMap::new())),
            interaction_storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn get_buddy(&self, buddy_id: &Uuid) -> Result<Buddy> {
        self.buddy_storage
            .read()
            .unwrap()
            .get(buddy_id)
            .context(format!("Looking for note with id {}", buddy_id))
            .map(|x| x.clone())
    }
    pub fn get_interaction(&self, interaction_id: &Uuid) -> Result<Interaction> {
        self.interaction_storage
            .read()
            .unwrap()
            .get(interaction_id)
            .context(format!("Looking for note with id {}", interaction_id))
            .map(|x| x.clone())
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
    fn create_interaction(&mut self, interaction: Interaction) -> Result<()> {
        self.interaction_storage
            .write()
            .unwrap()
            .insert(interaction.id.clone(), interaction.clone());
        Ok(())
    }
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>> {
        let mut users_buddies = HashMap::new();
        let storage = self.buddy_storage.read().unwrap();
        for buddy in storage.values() {
            if buddy.user_id == user_id && buddy.delete_timestamp.is_none() {
                users_buddies.insert(buddy.id.clone(), buddy.clone());
            }
        }
        Ok(users_buddies)
    }
    fn get_interactions(&self, user_id: Uuid) -> Result<HashMap<Uuid, Interaction>> {
        let mut users_interactions = HashMap::new();
        let storage = self.interaction_storage.read().unwrap();
        for interaction in storage.values() {
            if interaction.user_id == user_id && interaction.delete_timestamp.is_none() {
                users_interactions.insert(interaction.id.clone(), interaction.clone());
            }
        }
        Ok(users_interactions)
    }
    fn archive_buddy(&mut self, id: Uuid, _user_id: Uuid) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let mut buddy = self.get_buddy(&id).context("getting note to update")?;
        buddy.delete_timestamp = Some(Timestamp(now));
        Ok(())
    }

    fn archive_interaction(&mut self, id: Uuid, _user_id: Uuid) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let mut interaction = self
            .get_interaction(&id)
            .context("getting note to update")?;
        interaction.delete_timestamp = Some(Timestamp(now));
        Ok(())
    }
}
