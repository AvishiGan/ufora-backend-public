use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use rand::Rng;

// struct for storing otp
#[derive(serde::Serialize,serde::Deserialize,Debug)]
pub struct OTP {
    pub otp:String,
    pub created_at:DateTime<Utc>,
    pub expires_at:DateTime<Utc>
}

pub fn get_an_otp() -> Result<String,StatusCode> {

    let otp = rand::thread_rng().gen_range(100000..999999);

    Ok(otp.to_string())
}
