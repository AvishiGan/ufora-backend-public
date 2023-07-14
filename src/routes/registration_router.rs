use std::sync::Arc;

use crate::handlers::registration_handler;

use axum::{
    Router, 
    routing::{post, get}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_registration_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/register/undergraduate", post(registration_handler::register_an_undergraduate))
    
    
}