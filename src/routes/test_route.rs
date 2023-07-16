use std::sync::Arc;

use axum::{
    Router,
    routing::get
};
use surrealdb::{Surreal, engine::remote::ws::Client};



pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/test", get(test_handler))
}

async fn test_handler() -> String{
    "test".to_string()
}