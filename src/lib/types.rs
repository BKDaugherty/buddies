use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize, PartialEq, Eq, Hash)]
pub struct Location(pub String);
#[derive(Debug, Clone, Copy, Default, Deserialize, Queryable, Serialize, PartialEq, Eq, Hash)]
pub struct Timestamp(pub u64);

/// Display of NaiveDate yyyy-mm-dd
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize, PartialEq, Eq, Hash)]
pub struct Datestamp(pub String);

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Buddy {
    /// A unique id for your buddy
    pub id: Uuid,
    /// The human readable name of your buddy
    pub name: String,
    /// The birthday of your buddy
    pub birthday: Option<Datestamp>,
    /// The frequency with which you'd like to talk to your buddy
    pub cadence: Option<Duration>,
    /// Any notes you have about your buddy
    pub notes: String,
    /// Where your buddy is
    pub location: Option<Location>,
    /// The last time you contacted your buddy
    pub last_contacted: Datestamp,
    /// The time in which this buddy was registered in the DB
    pub create_timestamp: Timestamp,
    /// The last time this record was updated
    pub last_update_timestamp: Timestamp,
    /// The time in which this buddy was deleted
    pub delete_timestamp: Option<Timestamp>,
    /// The id of whoever's buddy this is
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Interaction {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct ArchiveBuddyRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct ArchiveInteractionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateBuddyRequest {
    /// The id of the user who is asking to register a buddy
    pub user_id: Uuid,
    /// The human readable name of your buddy
    pub name: String,
    /// The birthday of your buddy
    pub birthday: Option<Datestamp>,
    /// The frequency with which you'd like to talk to your buddy
    pub cadence: Option<Duration>,
    /// Any notes you have about your buddy
    pub notes: String,
    /// Where your buddy is
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateInteractionRequest {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct GetUserDataRequest {
    pub user_id: Uuid,
}
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
#[derive(Debug, Clone, Deserialize, Queryable, Serialize)]
pub struct CreateBuddyResponse {
    /// The buddy you just created
    pub buddy: Buddy,
}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct CreateInteractionResponse {}
#[derive(Debug, Clone, Deserialize, Queryable, Serialize)]
pub struct GetUserDataResponse {
    /// Map from buddy_id to buddy object
    pub buddies: HashMap<Uuid, Buddy>,
    pub interactions: HashMap<Uuid, Interaction>,
}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct LoginResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct SignUpResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct UpdateBuddyResponse {}
#[derive(Debug, Clone, Default, Deserialize, Queryable, Serialize)]
pub struct UpdateInteractionResponse {}
