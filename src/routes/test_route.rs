use std::sync::Arc;

use axum::{
    Router,
    routing::get, http::StatusCode
};
use surrealdb::{Surreal, engine::remote::ws::Client};



pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/test", get(test_handler))
}

async fn test_handler() -> Result<String,StatusCode> {
    crate::services::email::send_email("Receiver <sineththamuditha@gmail.com>","Hello from Ufora".to_string(),"This is the body of the email".to_string()).await?;
    Ok("Fit".to_owned())
}