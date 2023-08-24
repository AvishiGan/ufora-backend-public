use std::sync::Arc;

use crate::handlers::registration_handlers;

use axum::{
    Router, 
    routing::post
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_registration_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/register/:usertype", post(registration_handlers::register_a_user))
        .route("/api/register/undergraduate/university", post(registration_handlers::add_university_details))
    
    
}