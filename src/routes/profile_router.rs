// #![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

// use chrono::prelude::*;

use crate::handlers::profile_handlers::{create_profile, get_profile, get_user_profile, update_profile};
use crate::handlers::post_handlers::get_posts_for_profile;


pub fn get_profile_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/profile/create", post(create_profile))
        .route("/api/profile/retrieveProfileByID", get(get_profile))
        .route("/api/profile/retrieveProfile", get(get_user_profile))
        .route("/api/profile/updateProfile", put(update_profile))
        .route("/api/profile/get/post",get(get_posts_for_profile))
}
 