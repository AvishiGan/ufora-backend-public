use axum::{
    http::StatusCode, 
    response::IntoResponse
};

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
    exp: usize,
    username: Option<String>,
}

pub async fn get_jwt() -> Result<String,StatusCode> {

    let now = chrono::Utc::now().timestamp() as usize;

    Ok(encode(
        &Header::default(),
        &Claim {
            iat: now,
            exp: now + 60 * 60 * 24 * 30,
            username:None
        }, 
        &EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)

}

pub async fn verify_jwt(token: String) -> Result<Claim,StatusCode> {
    
    let token_msg = decode::<Claim>(
        &token,
        &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()), 
        &Validation::new(Algorithm::HS256))
        .map_err(|e| match e.kind() {
        _ => {println!("{:?}",e); StatusCode::INTERNAL_SERVER_ERROR}
    } )?;

    Ok(token_msg.claims)

}