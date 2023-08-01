
use std::sync::Arc;

use crate::handlers::forgot_password_handlers;

use axum::{
    Router, 
    routing::post
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_forgot_password_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/password/reset/otp/request", post(forgot_password_handlers::verify_email_and_send_otp))
        .route("/password/reset/otp/verify", post(forgot_password_handlers::verify_forgot_password_otp))
        .route("/password/reset", post(forgot_password_handlers::reset_password))
    
    
}