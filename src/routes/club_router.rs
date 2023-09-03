use std::sync::Arc;

use crate::{
    handlers::{
        club_handlers::{club_middleware_check, create_a_club_account, verify_club_email},
        login_handlers::club_login,
    },
    middlewares,
};

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_club_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/club/middleware", get(club_middleware_check))
        .layer(middleware::from_fn(
            middlewares::club_auth::validate_club_token,
        ))
        .route("/api/club/login/:club_id", post(club_login))
        .route("/api/club/create", post(create_a_club_account))
        .route("/api/club/email/verification", post(verify_club_email))
}
