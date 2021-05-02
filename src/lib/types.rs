use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct ArchiveBuddyRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct ArchiveInteractionRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateBuddyRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateInteractionRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct GetBuddiesRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct GetInteractionsRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct LoginRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct SignUpRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct UpdateBuddyRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct UpdateInteractionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct ArchiveBuddyResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct ArchiveInteractionResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateBuddyResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateInteractionResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct GetBuddiesResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct GetInteractionsResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct LoginResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct SignUpResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct UpdateBuddyResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct UpdateInteractionResponse {}
