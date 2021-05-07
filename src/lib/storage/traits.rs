use crate::lib::types::{Buddy, Interaction};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

pub trait BuddiesStore: Send + Sync + Clone + 'static {
    fn create_buddy(&mut self, buddy: Buddy) -> Result<()>;
    fn create_interaction(&mut self, interaction: Interaction) -> Result<()>;
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>>;
    fn get_interactions(&self, user_id: Uuid) -> Result<HashMap<Uuid, Interaction>>;
}
