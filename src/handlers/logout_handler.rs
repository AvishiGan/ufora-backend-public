use axum::http::StatusCode;
use axum_extra::extract::CookieJar;

use crate::services::jwt;

pub async fn logout(
    cookiejar: CookieJar
) -> Result<(),StatusCode> {
    let token = cookiejar.get("_secure-jwt").unwrap().value().to_string();

    let token = jwt::verify_jwt(token).await ?;

    println!("token: {:?}", token);

    Ok(())
}