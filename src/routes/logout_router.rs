use std::sync::Arc;

use crate::handlers::logout_handlers;

use axum::{
    Router, 
    routing::post
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_logout_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/logout", post(logout_handlers::logout))
    
    
}