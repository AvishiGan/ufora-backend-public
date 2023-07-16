use std::sync::Arc;

use crate::handlers::logout_handler;

use axum::{
    Router, 
    routing::{post, get}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_logout_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/logout", post(logout_handler::logout))
    
    
}