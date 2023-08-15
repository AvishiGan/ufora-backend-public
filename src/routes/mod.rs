mod login_router;
mod logout_router;
mod registration_router;
mod profile_router;
mod test_route;
mod verification_router;
mod forgot_password_router;
mod post_router;
mod blog_router;
mod project_router;

use std::sync::Arc;

use axum::{ Router, middleware, http::Method };
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{ CorsLayer, Any };

use crate::middlewares;

use login_router::get_login_router;
use logout_router::get_logout_router;
use registration_router::get_registration_router;
use verification_router::get_verification_router;
use forgot_password_router::get_forgot_password_router;
use post_router::get_post_router;
use blog_router::get_blog_router;
use project_router::get_project_router;
use surrealdb::{ Surreal, engine::remote::ws::Client };

use self::profile_router::get_profile_router;

pub fn get_router() -> Router<Arc<Surreal<Client>>> {
    let cors = CorsLayer::new().allow_methods(vec![Method::GET, Method::POST]).allow_origin(Any);

    Router::new()
        .merge(get_project_router())
        .merge(get_blog_router())
        .merge(get_post_router())
        .merge(get_logout_router())
        .merge(get_profile_router())
        .merge(test_route::get_test_router())
        .layer(middleware::from_fn(middlewares::auth::validate_jwt))
        .merge(get_login_router())
        .merge(get_forgot_password_router())
        .merge(get_registration_router())
        .merge(get_verification_router())
        .layer(CookieManagerLayer::new())
        .layer(cors)
}
