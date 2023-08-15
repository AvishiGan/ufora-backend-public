// #![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

// use chrono::prelude::*;

use crate::handlers::profile_handlers::{create_profile, get_profile, get_user_profile};



pub fn get_profile_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/profile/create", post(create_profile))
        .route("/api/profile/retrieveProfileByID", get(get_profile))
        .route("/api/profile/retrieveProfile", get(get_user_profile))
        // .route("/api/profile/create/:id", post(create_profile))
}
 