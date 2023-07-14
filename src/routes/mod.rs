mod login_router;
mod logout_router;
mod test_route;

use std::sync::Arc;

use axum::{
    Router, 
    middleware
};
use tower_cookies::CookieManagerLayer;

use crate::middlewares;

use login_router::get_login_router;
use logout_router::get_logout_router;
use surrealdb::{Surreal, engine::remote::ws::Client};


pub fn get_router() -> Router<Arc<Surreal<Client>>> {
    
    Router::new()
    .merge(test_route::get_test_router())
    .layer(middleware::from_fn(middlewares::auth::validate_jwt))
    .merge(get_login_router())
    .merge(get_logout_router())
    .layer(CookieManagerLayer::new())
}