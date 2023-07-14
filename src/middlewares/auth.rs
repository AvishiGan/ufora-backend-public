use axum::{http::{Request, StatusCode}, middleware::Next, response::Response,};
use axum_extra::extract::CookieJar;

use crate::services::jwt;

pub async fn validate_jwt<T>(
    cookiejar: CookieJar,
    request: Request<T>,
    next: Next<T>
) -> Result<Response, StatusCode> {

    if let Some(cookie) = cookiejar.get("_Secure-jwt") {
        let token = cookie.value().to_string();

        let token = jwt::verify_jwt(token).await ?;

        println!("token: {:?}", token);

        Ok(next.run(request).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
}