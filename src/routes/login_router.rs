
use std::sync::Arc;

use crate::handlers::login_handlers;

use axum::{
    Router, 
    routing::{post, get}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_login_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/login", post(login_handlers::login_via_platform))
    
    
}




