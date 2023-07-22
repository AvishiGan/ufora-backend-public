use std::sync::Arc;

use crate::handlers::registration_handlers;

use axum::{
    Router, 
    routing::post
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_registration_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/register/undergraduate", post(registration_handlers::register_an_undergraduate))
        .route("/register/company", post(registration_handlers::register_a_company))
    
    
}