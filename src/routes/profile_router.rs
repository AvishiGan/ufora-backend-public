// #![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

// use chrono::prelude::*;

// use crate::handlers::post_handlers::get_posts_for_profile;
use crate::handlers::profile_handlers::{create_profile, get_user_profile, update_profile, get_all_profiles};

pub fn get_profile_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/profile/create", post(create_profile))
        .route("/api/profile/retrieveProfile", get(get_user_profile))
    .route("/api/profile/updateProfile", put(update_profile))
    .route("/api/profile/allProfiles", get(get_all_profiles))
    // .route("/api/profile/get/post",get(get_posts_for_profile))
}
