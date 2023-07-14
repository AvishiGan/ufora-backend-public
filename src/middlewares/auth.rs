use axum::{http::{Request, StatusCode}, middleware::Next, response::Response,};
use tower_cookies::Cookies;

use crate::services::jwt;

pub async fn validate_jwt<T>(
    cookie: Cookies,
    request: Request<T>,
    next: Next<T>
) -> Result<Response, StatusCode> {

    let cookiejar = cookie;

    println!("cookiejar: {:?}", cookiejar);

    if let Some(cookie) = cookiejar.get("_secure-jwt") {
        let token = cookie.value().to_string();

        let token = jwt::verify_jwt(token).await ?;

        println!("token: {:?}", token);

        Ok(next.run(request).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
}