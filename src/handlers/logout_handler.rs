use axum::http::StatusCode;
use tower_cookies::{Cookies};


use crate::services::jwt;

pub async fn logout(
    cookie: Cookies
) -> Result<(),StatusCode> {
    let token = cookie.get("_secure-jwt").unwrap().value().to_string();

    let token = jwt::verify_jwt(token).await.unwrap() ;

    println!("token: {:?}", token);

    Ok(())
}