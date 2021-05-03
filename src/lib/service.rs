use crate::lib::storage::BuddiesStore;
use crate::lib::types::{
    ArchiveBuddyRequest, ArchiveBuddyResponse, ArchiveInteractionRequest,
    ArchiveInteractionResponse, Buddy, CreateBuddyRequest, CreateBuddyResponse,
    CreateInteractionRequest, CreateInteractionResponse, Datestamp, GetBuddiesRequest,
    GetBuddiesResponse, GetInteractionsRequest, GetInteractionsResponse, Location, LoginRequest,
    LoginResponse, SignUpRequest, SignUpResponse, Timestamp, UpdateBuddyRequest,
    UpdateBuddyResponse, UpdateInteractionRequest, UpdateInteractionResponse,
};
use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveDateTime};
use std::convert::TryInto;
use std::time::SystemTime;
use uuid::Uuid;

pub trait BuddiesService: Send + Sync + Clone + 'static {
    fn login(&self, request: LoginRequest) -> Result<LoginResponse>;
    fn sign_up(&mut self, request: SignUpRequest) -> Result<SignUpResponse>;

    // Buddy CRUD
    fn create_buddy(&mut self, request: CreateBuddyRequest) -> Result<CreateBuddyResponse>;
    fn get_buddies(&self, request: GetBuddiesRequest) -> Result<GetBuddiesResponse>;
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<UpdateBuddyResponse>;
    fn archive_buddy(&mut self, request: ArchiveBuddyRequest) -> Result<ArchiveBuddyResponse>;

    // Interaction CRUD
    fn create_interaction(
        &mut self,
        request: CreateInteractionRequest,
    ) -> Result<CreateInteractionResponse>;
    fn get_interactions(&self, request: GetInteractionsRequest) -> Result<GetInteractionsResponse>;
    fn update_interaction(
        &mut self,
        request: UpdateInteractionRequest,
    ) -> Result<UpdateInteractionResponse>;
    fn archive_interaction(
        &mut self,
        request: ArchiveInteractionRequest,
    ) -> Result<ArchiveInteractionResponse>;
}

#[derive(Clone)]
pub struct RequestHandler<S> {
    pub storage: S,
}

impl<S: BuddiesStore> RequestHandler<S> {
    pub fn new(storage: S) -> RequestHandler<S> {
        RequestHandler { storage }
    }
}

impl<S: BuddiesStore> BuddiesService for RequestHandler<S> {
    fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        todo!()
    }
    fn sign_up(&mut self, request: SignUpRequest) -> Result<SignUpResponse> {
        todo!()
    }

    fn create_buddy(&mut self, request: CreateBuddyRequest) -> Result<CreateBuddyResponse> {
        let buddy_id = Uuid::new_v4();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let date_time = NaiveDateTime::from_timestamp(now.try_into().unwrap(), 0);
        let buddy = Buddy {
            id: buddy_id,
            name: request.name,
            birthday: request.birthday,
            cadence: request.cadence,
            notes: request.notes,
            location: request.location,
            user_id: request.user_id,
            last_contacted: Datestamp(format!("{}", date_time.date())),
            create_timestamp: Timestamp(now),
            last_update_timestamp: Timestamp(now),
            delete_timestamp: None,
        };

        self.storage
            .create_buddy(buddy.clone())
            .context(format!("Creating buddy with id {}", buddy_id))?;

        Ok(CreateBuddyResponse { buddy })
    }
    fn get_buddies(&self, request: GetBuddiesRequest) -> Result<GetBuddiesResponse> {
        let buddies = self
            .storage
            .get_buddies(request.user_id)
            .context(format!("getting buddies under id {}", request.user_id))?;

        Ok(GetBuddiesResponse { buddies })
    }
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<UpdateBuddyResponse> {
        todo!()
    }
    fn archive_buddy(&mut self, request: ArchiveBuddyRequest) -> Result<ArchiveBuddyResponse> {
        todo!()
    }

    fn create_interaction(
        &mut self,
        request: CreateInteractionRequest,
    ) -> Result<CreateInteractionResponse> {
        todo!()
    }
    fn get_interactions(&self, request: GetInteractionsRequest) -> Result<GetInteractionsResponse> {
        todo!()
    }
    fn update_interaction(
        &mut self,
        request: UpdateInteractionRequest,
    ) -> Result<UpdateInteractionResponse> {
        todo!()
    }
    fn archive_interaction(
        &mut self,
        request: ArchiveInteractionRequest,
    ) -> Result<ArchiveInteractionResponse> {
        todo!()
    }
}
