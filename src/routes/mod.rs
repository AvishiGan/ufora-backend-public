mod login_router;

use std::sync::Arc;

use axum::Router;

use login_router::get_login_router;
use surrealdb::{Surreal, engine::remote::ws::Client};


pub fn get_router() -> Router<Arc<Surreal<Client>>> {
    
    Router::new()
    .merge(get_login_router())
}