use super::schema::{buddies, interactions, users};
use crate::lib::types::{
    Buddy, Datestamp, Interaction, Location, Timestamp, UpdateBuddyRequest,
    UpdateInteractionRequest, User,
};
use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::time::SystemTime;
use uuid::Uuid;

/// Our DB representation of a buddy
#[derive(Queryable)]
pub struct DBBuddy {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub notes: String,
    pub last_contacted: String,
    pub birthday: Option<String>,
    // TODO - Cadence
    pub location: Option<String>,
    pub create_timestamp: String,
    pub last_update_timestamp: String,
    pub delete_timestamp: Option<String>,
    pub user_uuid: String,
}

impl TryFrom<DBBuddy> for Buddy {
    type Error = anyhow::Error;

    fn try_from(buddy: DBBuddy) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&buddy.uuid).context("Parsing buddy id")?;
        let user_id = Uuid::parse_str(&buddy.user_uuid).context("parsing buddy's user id")?;
        let delete_timestamp = match buddy.delete_timestamp {
            Some(x) => Some(Timestamp(x.parse().context("Parsing delete timestamp")?)),
            None => None,
        };

        Ok(Buddy {
            id,
            user_id,
            name: buddy.name,
            birthday: buddy.birthday.map(Datestamp),
            notes: buddy.notes,
            last_contacted: Datestamp(buddy.last_contacted),
            create_timestamp: Timestamp(
                buddy
                    .create_timestamp
                    .parse()
                    .context("parsing create timestamp")?,
            ),
            last_update_timestamp: Timestamp(
                buddy
                    .last_update_timestamp
                    .parse()
                    .context("parsing last update timestamp")?,
            ),
            delete_timestamp,
            location: buddy.location.map(Location),
            cadence: None,
        })
    }
}

#[derive(Insertable)]
#[table_name = "buddies"]
pub struct NewBuddy {
    pub uuid: String,
    pub name: String,
    pub notes: String,
    pub last_contacted: String,
    pub location: Option<String>,
    pub birthday: Option<String>,
    pub create_timestamp: String,
    pub last_update_timestamp: String,
    pub user_uuid: String,
}

#[derive(AsChangeset, Default)]
#[table_name = "buddies"]
pub struct DBUpdateBuddy {
    pub last_update_timestamp: String,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub last_contacted: Option<String>,
    pub location: Option<String>,
    pub birthday: Option<String>,
    pub delete_timestamp: Option<String>,
}

impl DBUpdateBuddy {
    pub fn archive() -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        Ok(Self {
            last_update_timestamp: format!("{}", now),
            delete_timestamp: Some(format!("{}", now)),
            ..DBUpdateBuddy::default()
        })
    }
    pub fn update(request: UpdateBuddyRequest) -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        Ok(Self {
            name: request.name,
            notes: request.notes,
            birthday: request.birthday.map(|x| x.0),
            last_contacted: request.last_contacted.map(|x| x.0),
            location: request.location.map(|x| x.0),
            last_update_timestamp: format!("{}", now),
            ..DBUpdateBuddy::default()
        })
    }
}

impl TryFrom<Buddy> for NewBuddy {
    type Error = anyhow::Error;
    fn try_from(buddy: Buddy) -> Result<Self, Self::Error> {
        Ok(NewBuddy {
            uuid: buddy.id.to_string(),
            name: buddy.name,
            notes: buddy.notes,
            last_contacted: buddy.last_contacted.0,
            create_timestamp: buddy.create_timestamp.0.to_string(),
            last_update_timestamp: buddy.last_update_timestamp.0.to_string(),
            birthday: buddy.birthday.map(|b| b.0),
            location: buddy.location.map(|b| b.0),
            user_uuid: buddy.user_id.to_string(),
        })
    }
}

/// Our DB repr of an interaction
#[derive(Queryable)]
pub struct DBInteraction {
    pub id: i32,
    pub uuid: String,
    pub notes: String,
    pub participants: Vec<String>,
    pub date: Option<String>,
    pub create_timestamp: String,
    pub last_update_timestamp: String,
    pub delete_timestamp: Option<String>,
    pub user_uuid: String,
}

#[derive(Insertable)]
#[table_name = "interactions"]
pub struct NewInteraction {
    pub uuid: String,
    pub notes: String,
    pub participants: Vec<String>,
    pub date: Option<String>,
    pub create_timestamp: String,
    pub last_update_timestamp: String,
    pub user_uuid: String,
}

#[derive(AsChangeset, Default)]
#[table_name = "interactions"]
pub struct DBUpdateInteraction {
    pub last_update_timestamp: String,
    pub notes: Option<String>,
    pub date: Option<String>,
    pub participants: Option<Vec<String>>,
    pub delete_timestamp: Option<String>,
}

impl DBUpdateInteraction {
    pub fn archive() -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        Ok(Self {
            last_update_timestamp: format!("{}", now),
            delete_timestamp: Some(format!("{}", now)),
            ..DBUpdateInteraction::default()
        })
    }

    pub fn update(request: UpdateInteractionRequest) -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let participants = match request.participants {
            Some(participant_uuids) => {
                let mut participants = Vec::new();
                for uuid in participant_uuids {
                    participants.push(uuid.to_string());
                }
                Some(participants)
            }
            None => None,
        };

        Ok(Self {
            notes: request.notes,
            date: request.date.map(|x| x.0),
            participants,
            last_update_timestamp: format!("{}", now),
            delete_timestamp: None,
        })
    }
}

impl TryFrom<Interaction> for NewInteraction {
    type Error = anyhow::Error;
    fn try_from(interaction: Interaction) -> Result<Self, Self::Error> {
        let mut participants = Vec::new();
        for p in interaction.participants {
            participants.push(p.to_string())
        }

        Ok(NewInteraction {
            uuid: interaction.id.to_string(),
            notes: interaction.notes,
            participants,
            date: interaction.date.map(|d| d.0),
            create_timestamp: interaction.create_timestamp.0.to_string(),
            last_update_timestamp: interaction.last_update_timestamp.0.to_string(),
            user_uuid: interaction.user_id.to_string(),
        })
    }
}

impl TryFrom<DBInteraction> for Interaction {
    type Error = anyhow::Error;

    fn try_from(interaction: DBInteraction) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&interaction.uuid).context("Parsing interaction id")?;
        let user_id =
            Uuid::parse_str(&interaction.user_uuid).context("parsing interaction's user id")?;
        let delete_timestamp = match interaction.delete_timestamp {
            Some(x) => Some(Timestamp(x.parse().context("Parsing delete timestamp")?)),
            None => None,
        };

        let mut participants = HashSet::new();
        for p in interaction.participants {
            let p_uuid =
                Uuid::parse_str(&p).context("Parsing uuid for participant of interaction")?;
            if participants.contains(&p_uuid) {
                return Err(anyhow!(
                    "Interaction {} has duplicate participants!",
                    interaction.uuid
                ));
            }
            participants.insert(p_uuid);
        }
        Ok(Interaction {
            id,
            notes: interaction.notes,
            participants,
            date: interaction.date.map(Datestamp),
            create_timestamp: Timestamp(
                interaction
                    .create_timestamp
                    .parse()
                    .context("parsing create timestamp")?,
            ),
            last_update_timestamp: Timestamp(
                interaction
                    .last_update_timestamp
                    .parse()
                    .context("parsing last_update timestamp")?,
            ),
            delete_timestamp,
            user_id,
        })
    }
}

#[derive(Queryable, Debug)]
pub struct DBUser {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub user_uuid: String,
    pub create_timestamp: String,
}

impl TryFrom<DBUser> for User {
    type Error = anyhow::Error;

    fn try_from(user: DBUser) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&user.user_uuid).context("Parsing user id")?;
        Ok(Self {
            id,
            email: user.email,
            password: user.password,
            create_timestamp: Timestamp(
                user.create_timestamp
                    .parse()
                    .context("parsing create timestamp")?,
            ),
            last_update_timestamp: Timestamp(
                user.create_timestamp
                    .parse()
                    .context("parsing last_update timestamp")?,
            ),
        })
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub user_id: String,
    pub create_timestamp: String,
}
