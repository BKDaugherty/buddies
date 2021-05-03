use crate::lib::types::{Buddy, CreateBuddyRequest};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

pub trait BuddiesStore: Send + Sync + Clone + 'static {
    fn create_buddy(&mut self, buddy: Buddy) -> Result<()>;
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>>;
}
