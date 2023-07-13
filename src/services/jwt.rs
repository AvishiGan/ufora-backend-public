use axum::{
    http::StatusCode, 
    response::IntoResponse
};
use axum_extra::extract::cookie::Cookie;
use jsonwebtoken::{
    encode,
    decode,
    EncodingKey, 
    Algorithm, 
    DecodingKey, 
    Validation, Header
};
use dotenvy_macro::dotenv;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claim {
    iat: usize,
}

pub async fn get_jwt() -> Result<String,StatusCode> {

    let now = chrono::Utc::now().timestamp() as usize;

    Ok(encode(
        &Header::default(),
        &Claim {
            iat: now,
        },
        &EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)

}