use std::sync::Arc;

use crate::handlers::verification_handlers;

use axum::{
    Router, 
    routing::post
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_verification_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/otp/request", post(verification_handlers::send_otp_to_email))
        .route("/otp/verify/email", post(verification_handlers::verify_otp))
        .route("/otp/verify/university/email", post(verification_handlers::verify_otp_university_email))
        
    
    
}