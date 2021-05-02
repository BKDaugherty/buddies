use crate::lib::storage::BuddiesStore;
use crate::lib::types::{
    ArchiveBuddyRequest, ArchiveBuddyResponse, ArchiveInteractionRequest,
    ArchiveInteractionResponse, CreateBuddyRequest, CreateBuddyResponse, CreateInteractionRequest,
    CreateInteractionResponse, GetBuddiesRequest, GetBuddiesResponse, GetInteractionsRequest,
    GetInteractionsResponse, LoginRequest, LoginResponse, SignUpRequest, SignUpResponse,
    UpdateBuddyRequest, UpdateBuddyResponse, UpdateInteractionRequest, UpdateInteractionResponse,
};
use anyhow::Result;

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
        todo!()
    }
    fn get_buddies(&self, request: GetBuddiesRequest) -> Result<GetBuddiesResponse> {
        todo!()
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
