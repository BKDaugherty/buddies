use super::schema::buddies;
use crate::lib::types::{Buddy, Datestamp, Location, Timestamp};
use anyhow::{anyhow, Context};
use std::collections::HashSet;
use std::convert::TryFrom;
use uuid::Uuid;

/// Our DB representaiton of a buddy
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
    pub user_id: String,
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
            user_id: buddy.user_id.to_string(),
        })
    }
}
