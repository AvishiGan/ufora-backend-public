mod login_router;

use axum::Router;

use login_router::get_login_router;


pub fn get_router() -> Router {
    
    Router::new()
    .merge(get_login_router())
}