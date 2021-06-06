use super::models::{
    DBBuddy, DBInteraction, DBUpdateBuddy, DBUpdateInteraction, NewBuddy, NewInteraction,
};
use super::schema::{buddies, interactions};
use crate::lib::storage::traits::{AuthStore, BuddiesStore};
use crate::lib::types::{
    Buddy, CreateUserRequest, Interaction, LoginRequest, UpdateBuddyRequest,
    UpdateInteractionRequest, User,
};
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
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

impl AuthStore for PsqlBuddiesStore {
    fn create_user(&mut self, _request: CreateUserRequest) -> Result<()> {
        todo!()
    }
    fn get_user(&self, _request: LoginRequest) -> Result<User> {
        todo!()
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
        let conn = self.get_db_conn()?;
        let interaction_uuid = interaction.id.clone();
        let new_interaction_request = NewInteraction::try_from(interaction).context(format!(
            "attempting to create insert statement for interaction with uuid {}",
            interaction_uuid
        ))?;
        diesel::insert_into(interactions::table)
            .values(&new_interaction_request)
            .execute(&conn)
            .context(format!(
                "Error attempting to persist interaction in db with uuid {}",
                interaction_uuid
            ))?;
        Ok(())
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
        let user_id_string = user_id.to_string();
        let conn = self.get_db_conn()?;
        let db_interactions = interactions::dsl::interactions
            .filter(interactions::dsl::user_uuid.eq(&user_id_string))
            .load::<DBInteraction>(&conn)
            .context(format!("Looking for user {}", user_id_string))?;
        let mut resulting_map = HashMap::new();
        let interactions: Vec<Interaction> = db_interactions
            .into_iter()
            .map(|db_interaction| Interaction::try_from(db_interaction))
            .collect::<Result<Vec<Interaction>>>()
            .context(format!("Reading interactions for {}", user_id_string))?;
        // TODO Stop the attack of the clones
        for interaction in interactions {
            resulting_map.insert(interaction.id.clone(), interaction.clone());
        }
        Ok(resulting_map)
    }
    fn archive_buddy(&mut self, id: Uuid, user_id: Uuid) -> Result<()> {
        let conn = self.get_db_conn()?;
        let update = DBUpdateBuddy::archive().context("Creating archive buddy request")?;
        diesel::update(
            buddies::dsl::buddies
                .filter(buddies::dsl::uuid.eq(id.to_string()))
                .filter(buddies::dsl::user_uuid.eq(user_id.to_string())),
        )
        .set(&update)
        .execute(&conn)
        .context(format!("Archiving buddy {} {}", id, user_id))?;
        Ok(())
    }
    fn archive_interaction(&mut self, id: Uuid, user_id: Uuid) -> Result<()> {
        let conn = self.get_db_conn()?;
        let update =
            DBUpdateInteraction::archive().context("Creating archive interaction request")?;
        diesel::update(
            interactions::dsl::interactions
                .filter(interactions::dsl::uuid.eq(id.to_string()))
                .filter(interactions::dsl::user_uuid.eq(user_id.to_string())),
        )
        .set(&update)
        .execute(&conn)
        .context(format!("Archiving interaction {} {}", id, user_id))?;
        Ok(())
    }
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<()> {
        let conn = self.get_db_conn()?;
        // TODO - save 2 clones by refactoring DBUpdateBuddy to take ownership of only a
        // portion of the updatebuddyrequest
        let buddy_id = request.buddy_id.clone();
        let user_id = request.user_id.clone();
        let update = DBUpdateBuddy::update(request).context("Creating update buddy request")?;
        diesel::update(
            buddies::dsl::buddies
                .filter(buddies::dsl::uuid.eq(buddy_id.to_string()))
                .filter(buddies::dsl::user_uuid.eq(user_id.to_string())),
        )
        .set(&update)
        .execute(&conn)
        .context(format!("Updating buddy {} {}", buddy_id, user_id))?;
        Ok(())
    }
    fn update_interaction(&mut self, request: UpdateInteractionRequest) -> Result<()> {
        let conn = self.get_db_conn()?;
        // TODO - save 2 clones by refactoring DBUpdateInteraction to take ownership of only a
        // portion of the updateinteractionrequest
        let interaction_id = request.interaction_id.clone();
        let user_id = request.user_id.clone();
        let update =
            DBUpdateInteraction::update(request).context("Creating update interaction request")?;
        diesel::update(
            interactions::dsl::interactions
                .filter(interactions::dsl::uuid.eq(interaction_id.to_string()))
                .filter(interactions::dsl::user_uuid.eq(user_id.to_string())),
        )
        .set(&update)
        .execute(&conn)
        .context(format!(
            "Updating interaction {} {}",
            interaction_id, user_id
        ))?;
        Ok(())
    }
}
