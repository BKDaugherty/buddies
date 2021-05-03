use crate::lib::service::{BuddiesService, RequestHandler};
use crate::lib::storage::BuddiesStore;
use crate::lib::types::{
    CreateBuddyRequest, CreateInteractionRequest, GetBuddiesRequest, LoginRequest, SignUpRequest,
    UpdateBuddyRequest, UpdateInteractionRequest,
};

use uuid::Uuid;
use warp::{filters::BoxedFilter, http, Filter, Reply};

async fn login<S: BuddiesStore>(
    request: LoginRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn sign_up<S: BuddiesStore>(
    request: SignUpRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn create_buddy<S: BuddiesStore>(
    request: CreateBuddyRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Error handling for warp
    match handler.create_buddy(request) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(_e) => Err(warp::reject::not_found()),
    }
}

async fn get_buddies<S: BuddiesStore>(
    user_id: Uuid,
    handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match handler.get_buddies(GetBuddiesRequest { user_id }) {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(_e) => Err(warp::reject::not_found()),
    }
}

async fn archive_buddy<S: BuddiesStore>(
    buddy_id: Uuid,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn update_buddy<S: BuddiesStore>(
    request: UpdateBuddyRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn create_interaction<S: BuddiesStore>(
    request: CreateInteractionRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn get_interactions<S: BuddiesStore>(
    user_id: Uuid,
    handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn archive_interaction<S: BuddiesStore>(
    interaction_id: Uuid,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

async fn update_interaction<S: BuddiesStore>(
    request: UpdateInteractionRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

/// This function links the service to warp's route handling
pub fn build_warp_routes<S: BuddiesStore>(
    handler: RequestHandler<S>,
) -> BoxedFilter<(impl Reply,)> {
    // TODO - Figure out how to do JWT in Warp and add auth to handlers that
    // need it
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "PUT", "POST"]);
    let handler_filter = warp::any().map(move || handler.clone());

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(login);

    let sign_up = warp::post()
        .and(warp::path("sign_up"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(sign_up);

    let create_buddy = warp::post()
        .and(warp::path("buddy"))
        // Only accept bodies smaller than 16kb... (because warp said so)
        // https://github.com/seanmonstar/warp/blob/master/examples/body.rs
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(create_buddy);

    let archive_buddy = warp::put()
        .and(warp::path("buddy"))
        .and(warp::path("archive"))
        .and(warp::path::param::<Uuid>())
        .and(handler_filter.clone())
        .and_then(archive_buddy);

    let update_buddy = warp::put()
        .and(warp::path("buddy"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(update_buddy);

    let get_buddies = warp::get()
        .and(warp::path("buddies"))
        .and(warp::path::param::<Uuid>())
        .and(handler_filter.clone())
        .and_then(get_buddies);

    let create_interaction = warp::post()
        .and(warp::path("interaction"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(create_interaction);

    let archive_interaction = warp::put()
        .and(warp::path("interaction"))
        .and(warp::path("archive"))
        .and(warp::path::param::<Uuid>())
        .and(handler_filter.clone())
        .and_then(archive_interaction);

    let update_interaction = warp::put()
        .and(warp::path("interaction"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(update_interaction);

    let get_interactions = warp::get()
        .and(warp::path("interactions"))
        .and(warp::path::param::<Uuid>())
        .and(handler_filter.clone())
        .and_then(get_interactions);

    let routes = login
        .or(sign_up)
        .or(create_buddy)
        .or(update_buddy)
        .or(get_buddies)
        .or(archive_buddy)
        .or(create_interaction)
        .or(update_interaction)
        .or(get_interactions)
        .or(archive_interaction)
        .with(cors)
        .boxed();
    routes
}
