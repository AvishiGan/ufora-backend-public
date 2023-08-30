use std::sync::Arc;

use crate::{handlers::club_handlers::{create_a_club_account, verify_club_email, club_middleware_check}, middlewares};

use axum::{
    routing::{post, get},
    Router, middleware,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_club_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/club/middleware", get(club_middleware_check))
        .layer(middleware::from_fn(middlewares::club_auth::validate_club_token))
        .route("/api/club/create", post(create_a_club_account))
        .route("/api/club/email/verification", post(verify_club_email))
}
