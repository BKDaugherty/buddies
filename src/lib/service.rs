use crate::lib::storage::{AuthStore, BuddiesStore};
use crate::lib::types::{
    ArchiveBuddyRequest, ArchiveBuddyResponse, ArchiveInteractionRequest,
    ArchiveInteractionResponse, AuthenticationRequest, AuthenticationResponse, Buddy,
    CreateBuddyRequest, CreateBuddyResponse, CreateInteractionRequest, CreateInteractionResponse,
    CreateUserRequest, Datestamp, GetUserDataRequest, GetUserDataResponse, Interaction,
    LoginRequest, LoginResponse, PublicUser, SignUpRequest, SignUpResponse, Timestamp,
    UpdateBuddyRequest, UpdateBuddyResponse, UpdateInteractionRequest, UpdateInteractionResponse,
    User,
};
use anyhow::{anyhow, Context, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::time::SystemTime;
use uuid::Uuid;

// y'know why not
const JWT_EXPIRATION_HOURS: i64 = 72;

pub trait AuthService: Send + Sync + Clone + 'static {
    fn login(&self, request: LoginRequest) -> Result<LoginResponse>;
    fn sign_up(&mut self, request: SignUpRequest) -> Result<SignUpResponse>;
    fn authenticate(&self, request: AuthenticationRequest) -> Result<AuthenticationResponse>;
}

pub trait BuddiesService: Send + Sync + Clone + 'static {
    // Buddy CRUD
    fn create_buddy(&mut self, request: CreateBuddyRequest) -> Result<CreateBuddyResponse>;
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<UpdateBuddyResponse>;
    fn archive_buddy(&mut self, request: ArchiveBuddyRequest) -> Result<ArchiveBuddyResponse>;
    fn get_user_data(&self, request: GetUserDataRequest) -> Result<GetUserDataResponse>;

    // Interaction CRUD
    fn create_interaction(
        &mut self,
        request: CreateInteractionRequest,
    ) -> Result<CreateInteractionResponse>;
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

impl<S: AuthStore> AuthService for AuthHandler<S> {
    fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        let mut storage_request = request;
        storage_request.password = self
            .hash(storage_request.password)
            .context("Creating password hash")?;

        let user = self
            .storage
            .get_user(storage_request)
            .context("Retrieving User")?;

        if let Err(e) = verify(request.password, &user.password) {
            return Err(anyhow!("Password mismatch - {:?}", e));
        }

        let user_uuid = user.id.clone();

        Ok(LoginResponse {
            user: PublicUser::from(user),
            jwt: self.create_jwt(user_uuid).context("Creating JWT")?,
        })
    }
    fn sign_up(&mut self, request: SignUpRequest) -> Result<SignUpResponse> {
        let user_id = Uuid::new_v4();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let password_hash = self
            .hash(request.password.clone())
            .context("Creating password hash")?;

        let user = User {
            id: user_id,
            email: request.email,
            password: password_hash,
            create_timestamp: Timestamp(now.clone()),
            last_update_timestamp: Timestamp(now),
        };

        self.storage
            .create_user(CreateUserRequest { user: user.clone() })
            .context("Creating User")?;

        let public_user = PublicUser::from(user);
        Ok(SignUpResponse { user: public_user })
    }
    fn authenticate(&self, request: AuthenticationRequest) -> Result<AuthenticationResponse> {
        let claims = self
            .decode_token(&request.json_web_token)
            .context("Parsing JWT")?;
        let user_uuid = claims.get_user_id().context("Parsing UUID")?;
        Ok(AuthenticationResponse { user_uuid })
    }
}

impl<S: BuddiesStore> BuddiesService for RequestHandler<S> {
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

    fn get_user_data(&self, request: GetUserDataRequest) -> Result<GetUserDataResponse> {
        let buddies = self
            .storage
            .get_buddies(request.user_id)
            .context("getting buddies")?;

        let interactions = self
            .storage
            .get_interactions(request.user_id)
            .context("getting interactions")?;

        Ok(GetUserDataResponse {
            buddies,
            interactions,
        })
    }
    fn archive_buddy(&mut self, request: ArchiveBuddyRequest) -> Result<ArchiveBuddyResponse> {
        self.storage
            .archive_buddy(request.id, request.user_id)
            .context("Attempting to archive buddy")?;
        Ok(ArchiveBuddyResponse {})
    }
    fn update_buddy(&mut self, request: UpdateBuddyRequest) -> Result<UpdateBuddyResponse> {
        self.storage
            .update_buddy(request)
            .context("updating buddy")?;
        Ok(UpdateBuddyResponse {})
    }
    fn create_interaction(
        &mut self,
        request: CreateInteractionRequest,
    ) -> Result<CreateInteractionResponse> {
        let interaction_id = Uuid::new_v4();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let interaction = Interaction {
            id: interaction_id,
            notes: request.notes,
            user_id: request.user_id,
            date: request.date,
            participants: request.participants,
            create_timestamp: Timestamp(now),
            last_update_timestamp: Timestamp(now),
            delete_timestamp: None,
        };

        self.storage
            .create_interaction(interaction.clone())
            .context(format!("Creating interaction with id {}", interaction_id))?;

        Ok(CreateInteractionResponse { interaction })
    }
    fn update_interaction(
        &mut self,
        request: UpdateInteractionRequest,
    ) -> Result<UpdateInteractionResponse> {
        self.storage
            .update_interaction(request)
            .context("updating interaction")?;
        Ok(UpdateInteractionResponse {})
    }
    fn archive_interaction(
        &mut self,
        request: ArchiveInteractionRequest,
    ) -> Result<ArchiveInteractionResponse> {
        self.storage
            .archive_interaction(request.id, request.user_id)
            .context("Attempting to archive interaction")?;
        Ok(ArchiveInteractionResponse {})
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn with_user_id(user_id: Uuid) -> Self {
        Claims {
            sub: user_id.to_string(),
            exp: (Local::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp() as usize,
        }
    }
    fn get_user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub).context(format!("Parsing sub of jwt {}", self.sub))
    }
}

#[derive(Clone, Debug)]
pub struct AuthHandler<S> {
    pub storage: S,
    pub secret: Vec<u8>,
    pub public: Vec<u8>,
}

impl<S: AuthStore> AuthHandler<S> {
    pub fn new<'a>(storage: S, secret: Vec<u8>, public: Vec<u8>) -> Result<AuthHandler<S>> {
        Ok(AuthHandler {
            storage,
            secret,
            public,
        })
    }

    pub fn hash(&self, password: String) -> Result<String> {
        match hash(password.clone(), DEFAULT_COST) {
            Ok(hash) => Ok(hash),
            Err(e) => Err(anyhow!("Failed to hash {} - {}", password, e)),
        }
    }

    pub fn create_jwt(&self, user_id: Uuid) -> Result<String> {
        let claims = Claims::with_user_id(user_id);
        let encoding_key = EncodingKey::from_rsa_pem(&self.secret).context("creating encoder")?;
        match encode(&Header::new(Algorithm::RS256), &claims, &encoding_key) {
            Ok(encoded) => Ok(encoded),
            Err(e) => Err(anyhow!(
                "Failed to create jwt for user_id {} - {}",
                user_id,
                e
            )),
        }
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        let decoding_key = DecodingKey::from_rsa_pem(&self.public).context("creating decoder")?;
        match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::RS256)) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => Err(anyhow!("Failed to decode token - {}", e)),
        }
    }
}
