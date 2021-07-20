use crate::lib::service::{AuthHandler, AuthService, BuddiesService, RequestHandler};
use crate::lib::storage::{AuthStore, BuddiesStore};
use crate::lib::types::{
    ArchiveBuddyRequest, ArchiveInteractionRequest, AuthenticationRequest, CreateBuddyRequest,
    CreateInteractionRequest, GetUserDataRequest, LoginRequest, SignUpRequest, UpdateBuddyRequest,
    UpdateInteractionRequest,
};
use log::error;
use serde::Serialize;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::{filters::BoxedFilter, Filter, Reply};

#[derive(Debug)]
enum ErrorType {
    Unknown(String),
}

#[derive(Debug)]
struct CustomError {
    error: ErrorType,
}

impl CustomError {
    fn unknown(message: String) -> Self {
        CustomError {
            error: ErrorType::Unknown(message),
        }
    }
}

impl warp::reject::Reject for CustomError {}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

async fn login<S: AuthStore>(
    request: LoginRequest,
    handler: AuthHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match handler.login(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn sign_up<S: AuthStore>(
    request: SignUpRequest,
    mut handler: AuthHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match handler.sign_up(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn create_buddy<S: BuddiesStore>(
    request: CreateBuddyRequest,
    auth_result: Result<Uuid, warp::Rejection>,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;

    match handler.create_buddy(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn get_user_data<S: BuddiesStore>(
    user_id: Uuid,
    auth_result: Result<Uuid, warp::Rejection>,
    handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;
    match handler.get_user_data(GetUserDataRequest { user_id }) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn archive_buddy<S: BuddiesStore>(
    request: ArchiveBuddyRequest,
    auth_result: Result<Uuid, warp::Rejection>,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;
    match handler.archive_buddy(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn update_buddy<S: BuddiesStore>(
    request: UpdateBuddyRequest,
    auth_result: Result<Uuid, warp::Rejection>,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;
    match handler.update_buddy(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn create_interaction<S: BuddiesStore>(
    request: CreateInteractionRequest,
    auth_result: Result<Uuid, warp::Rejection>,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;
    match handler.create_interaction(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn archive_interaction<S: BuddiesStore>(
    request: ArchiveInteractionRequest,
    auth_result: Result<Uuid, warp::Rejection>,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;
    match handler.archive_interaction(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

async fn update_interaction<S: BuddiesStore>(
    request: UpdateInteractionRequest,
    auth_result: Result<Uuid, warp::Rejection>,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user_uuid = auth_result?;
    match handler.update_interaction(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

fn authenticate<T: AuthStore>(
    handler: AuthHandler<T>,
    authorization_header: String,
) -> Result<Uuid, warp::Rejection> {
    let auth_header_components: Vec<&str> = authorization_header.split(' ').collect();

    let jwt = match &auth_header_components[..] {
        ["Bearer", jwt] => jwt.to_string(),
        bad_form => {
            return Err(warp::reject::custom(CustomError::unknown(format!(
                "Malformed Authorization Header {:?}",
                bad_form,
            ))));
        }
    };
    match handler.authenticate(AuthenticationRequest {
        json_web_token: jwt,
    }) {
        Ok(response) => Ok(response.user_uuid),
        Err(e) => Err(warp::reject::custom(CustomError::unknown(format!(
            "Failure {:?}",
            e
        )))),
    }
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_custom_rejection(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    if let Some(CustomError { error }) = err.find() {
        error!("Handling Error - {:?}", error);
        match error {
            ErrorType::Unknown(message) => {
                let code = StatusCode::INTERNAL_SERVER_ERROR;
                let json_reply = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message: message.into(),
                });
                Ok(warp::reply::with_status(json_reply, code))
            }
        }
    } else {
        error!("Passing along unknown Warp Rejection - {:?}", err);
        Err(err)
    }
}

/// This function links the service to warp's route handling
pub fn build_warp_routes<S: BuddiesStore, T: AuthStore>(
    auth_handler: AuthHandler<T>,
    handler: RequestHandler<S>,
) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Accept",
            "Accept-Encoding",
            "Access-Control-Request-Headers",
            "Access-Contorl-Request-Method",
            "Authorization",
            "Connection",
            "Content-Type",
            "Host",
            "Origin",
            "Referer",
            "Sec-Fetch-Dest",
            "Sec-Fetch-Mode",
            "User-Agent",
        ])
        .allow_methods(vec!["GET", "PUT", "POST"]);

    let auth_handler_filter = warp::any().map(move || auth_handler.clone());
    let auth_filter = auth_handler_filter
        .clone()
        .and(warp::header("Authorization"))
        .map(authenticate);

    let handler_filter = warp::any().map(move || handler.clone());

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_handler_filter.clone())
        .and_then(login);

    let sign_up = warp::post()
        .and(warp::path("sign_up"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_handler_filter.clone())
        .and_then(sign_up);

    let create_buddy = warp::post()
        .and(warp::path("buddy"))
        .and(warp::path("create"))
        // Only accept bodies smaller than 16kb... (because warp said so)
        // https://github.com/seanmonstar/warp/blob/master/examples/body.rs
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(create_buddy);

    let archive_buddy = warp::post()
        .and(warp::path("buddy"))
        .and(warp::path("archive"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(archive_buddy);

    let update_buddy = warp::post()
        .and(warp::path("buddy"))
        .and(warp::path("update"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(update_buddy);

    let create_interaction = warp::post()
        .and(warp::path("interaction"))
        .and(warp::path("create"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(create_interaction);

    let archive_interaction = warp::post()
        .and(warp::path("interaction"))
        .and(warp::path("archive"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(archive_interaction);

    let update_interaction = warp::post()
        .and(warp::path("interaction"))
        .and(warp::path("update"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(update_interaction);

    let get_user_data = warp::get()
        .and(warp::path("user"))
        .and(warp::path::param::<Uuid>())
        .and(auth_filter.clone())
        .and(handler_filter.clone())
        .and_then(get_user_data);

    let routes = login
        .or(sign_up)
        .or(create_buddy)
        .or(update_buddy)
        .or(archive_buddy)
        .or(create_interaction)
        .or(update_interaction)
        .or(archive_interaction)
        .or(get_user_data)
        .recover(handle_custom_rejection) // Added this and still am not getting what i think out of it...
        .with(cors)
        .boxed();
    routes
}
