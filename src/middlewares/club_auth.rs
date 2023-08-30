use axum::{http::{HeaderMap, Request}, middleware::Next, response::Response};
use reqwest::StatusCode;

pub async fn validate_club_token<T>(
    headers: HeaderMap,
    mut request: Request<T>,
    next: Next<T>
) -> Result<Response, StatusCode> {
    Ok(next.run(request).await)
}