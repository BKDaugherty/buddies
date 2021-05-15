use super::models::{DBBuddy, NewBuddy};
use super::schema::buddies;
use crate::lib::storage::traits::BuddiesStore;
use crate::lib::types::{Buddy, Interaction};
use anyhow::{anyhow, Context, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;
use std::collections::HashMap;
use std::convert::TryFrom;
use uuid::Uuid;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBCon = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(psql_str: &str) -> std::result::Result<DBPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(psql_str);
    Pool::builder().build(manager)
}

#[derive(Clone)]
pub struct PsqlBuddiesStore {
    db_pool: DBPool,
}

impl PsqlBuddiesStore {
    pub fn new(psql_str: &str) -> PsqlBuddiesStore {
        let db_pool = create_pool(psql_str).expect("Could not connect to database");
        PsqlBuddiesStore { db_pool }
    }

    pub fn get_db_conn(&self) -> Result<DBCon, PoolError> {
        self.db_pool.get()
    }
}

impl BuddiesStore for PsqlBuddiesStore {
    fn create_buddy(&mut self, buddy: Buddy) -> Result<()> {
        let conn = self.get_db_conn()?;
        let buddy_uuid = buddy.id.clone();
        let new_buddy_request = NewBuddy::try_from(buddy).context(format!(
            "attempting to create insert statement for buddy with uuid {}",
            buddy_uuid
        ))?;
        diesel::insert_into(buddies::table)
            .values(&new_buddy_request)
            .execute(&conn)
            .context(format!(
                "Error attempting to persist buddy in db with uuid {}",
                buddy_uuid
            ))?;
        Ok(())
    }
    fn create_interaction(&mut self, interaction: Interaction) -> Result<()> {
        todo!()
    }
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>> {
        let user_id_string = user_id.to_string();

        let conn = self.get_db_conn()?;
        let db_buddies = buddies::dsl::buddies
            .filter(buddies::dsl::user_uuid.eq(&user_id_string))
            .load::<DBBuddy>(&conn)
            .context(format!("Looking for user {}", user_id_string))?;
        let mut resulting_map = HashMap::new();
        let buddies: Vec<Buddy> = db_buddies
            .into_iter()
            .map(|db_buddy| Buddy::try_from(db_buddy))
            .collect::<Result<Vec<Buddy>>>()
            .context(format!("Reading buddies for {}", user_id_string))?;
        // TODO Stop the attack of the clones
        for buddy in buddies {
            resulting_map.insert(buddy.id.clone(), buddy.clone());
        }
        Ok(resulting_map)
    }
    fn get_interactions(&self, user_id: Uuid) -> Result<HashMap<Uuid, Interaction>> {
        todo!()
    }
}
