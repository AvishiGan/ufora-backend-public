use axum::{http::Request, middleware::Next, response::Response};
use reqwest::StatusCode;

pub async fn validate_club_token<T>(
    mut request: Request<T>,
    next: Next<T>
) -> Result<Response, StatusCode> {
    
    if request.headers().contains_key("Club-Authorization") {
        let authorization_header = request.headers().get("Club-Authorization").unwrap().to_str().unwrap().to_string();
        let club_token = crate::services::jwt::verify_club_jwt(authorization_header.to_string())?;
        let claim = crate::models::club_claim::ClubClaim::from(club_token);
        request.extensions_mut().insert(claim);
        Ok(next.run(request).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
}