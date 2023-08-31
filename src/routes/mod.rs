mod blog_router;
mod chat_router;
mod club_router;
mod forgot_password_router;
mod login_router;
mod logout_router;
mod post_router;
mod profile_router;
mod project_router;
mod registration_router;
mod test_route;
mod verification_router;

use std::sync::Arc;

use axum::{http::Method, middleware, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::middlewares;

use blog_router::get_blog_router;
use chat_router::get_chat_router;
use club_router::get_club_router;
use forgot_password_router::get_forgot_password_router;
use login_router::get_login_router;
use logout_router::get_logout_router;
use post_router::get_post_router;
use profile_router::get_profile_router;
use project_router::get_project_router;
use registration_router::get_registration_router;
use verification_router::get_verification_router;

use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_router() -> Router<Arc<Surreal<Client>>> {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        //merge test router -> for testing new features before adding
        .merge(test_route::get_test_router())
        // merge club router
        .merge(get_club_router())
        // merge project router
        .merge(get_project_router())
        // merge blog router
        .merge(get_blog_router())
        // merge post router
        .merge(get_post_router())
        // merge logout router
        .merge(get_logout_router())
        // merge profile router
        .merge(get_profile_router())
        // layer to validate jwt -> check whether user has access
        .layer(middleware::from_fn(middlewares::auth::validate_jwt))
        // merge chat router -> without authorization
        .merge(get_chat_router())
        // merge login router
        .merge(get_login_router())
        // merge forgot password router
        .merge(get_forgot_password_router())
        // merge registration router
        .merge(get_registration_router())
        // merge verification router
        .merge(get_verification_router())
        // layer to manage cookies
        .layer(CookieManagerLayer::new())
        // layer to allow cors
        .layer(cors)
}
