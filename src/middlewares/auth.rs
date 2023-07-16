use axum::{http::{Request, StatusCode}, middleware::Next, response::Response,};
use tower_cookies::Cookies;

use crate::services::jwt;

pub async fn validate_jwt<T>(
    cookies: Cookies,
    request: Request<T>,
    next: Next<T>
) -> Result<Response, StatusCode> {

    if let Some(cookie) = cookies.get("_Secure-jwt") {
        let token = cookie.value().to_string();

        let token = jwt::verify_jwt(token).await ?;

        Ok(next.run(request).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
}