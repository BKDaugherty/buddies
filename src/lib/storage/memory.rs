use super::traits::{AuthStore, BuddiesStore};
use crate::lib::types::{
    Buddy, CreateUserRequest, Interaction, LoginRequest, Timestamp, UpdateBuddyRequest,
    UpdateInteractionRequest, User,
};
use anyhow::{anyhow, Context, Result};
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
    /// Represents an "users" table
    user_storage: Arc<RwLock<HashMap<String, User>>>,
}

impl MemoryBuddiesStore {
    pub fn new() -> Self {
        Self {
            buddy_storage: Arc::new(RwLock::new(HashMap::new())),
            interaction_storage: Arc::new(RwLock::new(HashMap::new())),
            user_storage: Arc::new(RwLock::new(HashMap::new())),
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
    pub fn find_user(&self, email: &String) -> Option<User> {
        self.user_storage
            .read()
            .unwrap()
            .get(email)
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
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let mut buddy = self
            .get_buddy(&request.buddy_id)
            .context("getting note to update")?;
        if let Some(name) = request.name {
            buddy.name = name;
        }
        if let Some(notes) = request.notes {
            buddy.notes = notes;
        }
        if let Some(last_contacted) = request.last_contacted {
            buddy.last_contacted = last_contacted;
        }
        if let Some(location) = request.location {
            buddy.location = Some(location);
        }
        buddy.last_update_timestamp = Timestamp(now);
        Ok(())
    }
    fn update_interaction(&mut self, request: UpdateInteractionRequest) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let mut interaction = self
            .get_interaction(&request.interaction_id)
            .context("getting note to update")?;
        interaction.last_update_timestamp = Timestamp(now);
        if let Some(notes) = request.notes {
            interaction.notes = notes;
        }
        if let Some(date) = request.date {
            interaction.date = Some(date);
        }
        if let Some(participants) = request.participants {
            interaction.participants = participants;
        }
        Ok(())
    }
}

impl AuthStore for MemoryBuddiesStore {
    fn create_user(&mut self, request: CreateUserRequest) -> Result<()> {
        match self.find_user(&request.user.email) {
            Some(..) => {
                return Err(anyhow!("User already exists"));
            }
            None => {
                self.user_storage
                    .write()
                    .unwrap()
                    .insert(request.user.email.clone(), request.user);
            }
        }
        Ok(())
    }
    fn get_user(&self, request: &LoginRequest) -> Result<User> {
        match self.find_user(&request.email) {
            Some(user) => Ok(user),
            None => return Err(anyhow!("No user found for email {}", request.email)),
        }
    }
}
