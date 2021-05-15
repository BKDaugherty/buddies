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
	todo!()
    }
    fn create_interaction(&mut self, interaction: Interaction) -> Result<()> {
	todo!()
    }
    fn get_buddies(&self, user_id: Uuid) -> Result<HashMap<Uuid, Buddy>> {
	todo!()
    }
    fn get_interactions(&self, user_id: Uuid) -> Result<HashMap<Uuid, Interaction>> {
	todo!()
    }
}
