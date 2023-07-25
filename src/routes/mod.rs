mod login_router;
mod logout_router;
mod registration_router;
mod test_route;
mod verification_router;
mod forgot_password_router;

use std::sync::Arc;

use axum::{
    Router, 
    middleware
};
use tower_cookies::CookieManagerLayer;

use crate::middlewares;

use login_router::get_login_router;
use logout_router::get_logout_router;
use registration_router::get_registration_router;
use surrealdb::{Surreal, engine::remote::ws::Client};


pub fn get_router() -> Router<Arc<Surreal<Client>>> {
    
    Router::new()
    .merge(get_logout_router())
    .merge(test_route::get_test_router())
    .layer(middleware::from_fn(middlewares::auth::validate_jwt))
    .merge(get_login_router())
    .merge(forgot_password_router::get_forgot_password_router())
    .merge(get_registration_router())
    .merge(verification_router::get_verification_router())
    .layer(CookieManagerLayer::new())

}