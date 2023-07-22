use axum::http::StatusCode;

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
    user_id: String,
    user_type: String,
    iat: usize,
    exp: usize,
    username: Option<String>,
}

impl Claim {
    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
    pub fn get_user_type(&self) -> String {
        self.user_type.clone()
    }
}

pub async fn get_jwt(user_id: String,user_type:String) -> Result<String,StatusCode> {

    let now = chrono::Utc::now().timestamp() as usize;

    Ok(encode(
        &Header::default(),
        &Claim {
            user_id,
            user_type,
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
        _ => {println!("{:?}",e); StatusCode::BAD_REQUEST}
    } )?;

    Ok(token_msg.claims)

}