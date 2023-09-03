use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;

use crate::services::jwt;

pub async fn validate_jwt<T>(
    cookies: Cookies,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    if request.headers().contains_key("Authorization") {
        let authorization_header = request
            .headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let token =
            jwt::verify_jwt(authorization_header.split(" ").collect::<Vec<&str>>()[1].to_string())
                .await?;

        let claim = crate::models::user_claim::Claim::from(token);

        request.extensions_mut().insert(claim);

        Ok(next.run(request).await)
    } else if let Some(cookie) = cookies.get("_Secure-jwt") {
        let token = cookie.value().to_string();

        let token = jwt::verify_jwt(token).await?;

        let claim = crate::models::user_claim::Claim::from(token);

        request.extensions_mut().insert(claim);

        Ok(next.run(request).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}
