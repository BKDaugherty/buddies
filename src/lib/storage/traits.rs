use crate::lib::types::{
    Buddy, CreateUserRequest, Interaction, LoginRequest, UpdateBuddyRequest,
    UpdateInteractionRequest, User,
};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

pub trait BuddiesStore: Send + Sync + Clone + 'static {
    fn create_buddy(&mut self, buddy: Buddy) -> Result<()>;
    fn archive_buddy(&mut self, id: Uuid, user_id: Uuid) -> Result<()>;
    fn create_interaction(&mut self, interaction: Interaction) -> Result<()>;
    fn archive_interaction(&mut self, id: Uuid, user_id: Uuid) -> Result<()>;
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>>;
    fn get_interactions(&self, user_id: Uuid) -> Result<HashMap<Uuid, Interaction>>;
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<()>;
    fn update_interaction(&mut self, request: UpdateInteractionRequest) -> Result<()>;
}

pub trait AuthStore: Send + Sync + Clone + 'static {
    fn create_user(&mut self, request: CreateUserRequest) -> Result<()>;
    fn get_user(&self, request: &LoginRequest) -> Result<User>;
}
