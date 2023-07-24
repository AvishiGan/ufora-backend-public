
use std::sync::Arc;

use crate::handlers::forgot_password_handlers;

use axum::{
    Router, 
    routing::get
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_forgot_password_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/password/request/otp", get(forgot_password_handlers::verify_email_and_send_otp))
    
    
}