
use std::sync::Arc;

use axum::{
    Router, 
    routing::{post}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_login_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/login", post(login))
    
    
}

async fn login() {
    println!("login");
}


