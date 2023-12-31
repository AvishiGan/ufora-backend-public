use axum::http::StatusCode;
use chrono::prelude::*;

use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

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
    pub fn get_id(&self) -> String {
        self.user_id.clone()
    }
    pub fn get_user_type(&self) -> String {
        self.user_type.clone()
    }
}

// function to get jwt
pub async fn get_jwt(user_id: String, user_type: String) -> Result<String, StatusCode> {
    // get current time from local timezone
    let now = Utc
        .from_local_datetime(&chrono::Local::now().naive_local())
        .single()
        .unwrap()
        .timestamp() as usize;

    // encode jwt
    Ok(encode(
        &Header::default(),
        &Claim {
            user_id,
            user_type,
            iat: now,
            exp: now + 60 * 60 * 24 * 30,
            username: None,
        },
        &EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

// function to verify jwt
pub async fn verify_jwt(token: String) -> Result<Claim, StatusCode> {
    // decode jwt
    let token_msg = decode::<Claim>(
        &token,
        &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| match e.kind() {
        _ => {
            println!("{:?}", e);
            StatusCode::BAD_REQUEST
        }
    })?;

    // return decoded jwt
    Ok(token_msg.claims)
}

// claim struct for club jwt
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ClubClaim {
    pub club_id: String,
    pub position: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn get_club_jwt(club_id: String, position: String) -> Result<String, String> {
    // get current time from local timezone
    let now = Utc
        .from_local_datetime(&chrono::Local::now().naive_local())
        .single()
        .unwrap()
        .timestamp() as usize;

    // encode jwt
    Ok(encode(
        &Header::default(),
        &ClubClaim {
            club_id,
            position,
            iat: now,
            exp: now + 60 * 60 * 24 * 30,
        },
        &EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
    )
    .map_err(|e: jsonwebtoken::errors::Error| e.to_string())?)
}

pub fn verify_club_jwt(token: String) -> Result<ClubClaim, StatusCode> {
    let club_token_msg = decode::<ClubClaim>(
        &token,
        &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| match e.kind() {
        _ => {
            println!("{:?}", e);
            StatusCode::BAD_REQUEST
        }
    })?;
    Ok(club_token_msg.claims)
}

impl ClubClaim {
    pub fn get_club_id(&self) -> String {
        self.club_id.clone()
    }
    pub fn get_position(&self) -> String {
        self.position.clone()
    }
}
