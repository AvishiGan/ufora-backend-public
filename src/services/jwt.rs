use axum::http::StatusCode;
use chrono::prelude::*;

use jsonwebtoken::{
    encode,
    decode,
    EncodingKey, 
    Algorithm, 
    DecodingKey, 
    Validation, Header
};
use dotenvy_macro::dotenv;

// claim struct for jwt
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claim {
    user_id: String,
    user_type: String,
    iat: usize,
    exp: usize,
    username: Option<String>,
}

// implementation of claim struct to get user id and user type
impl Claim {
    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
    pub fn get_user_type(&self) -> String {
        self.user_type.clone()
    }
}

// function to get jwt
pub async fn get_jwt(user_id: String,user_type:String) -> Result<String,StatusCode> {

    // get current time from local timezone
    let now = Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap().timestamp() as usize;

    // encode jwt
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

// function to verify jwt
pub async fn verify_jwt(token: String) -> Result<Claim,StatusCode> {
    
    // decode jwt
    let token_msg = decode::<Claim>(
        &token,
        &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()), 
        &Validation::new(Algorithm::HS256))
        .map_err(|e| match e.kind() {
        _ => {println!("{:?}",e); StatusCode::BAD_REQUEST}
    } )?;

    // return decoded jwt
    Ok(token_msg.claims)

}