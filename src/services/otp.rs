use axum::http::StatusCode;
use chrono::prelude::*;
use rand::Rng;


pub fn get_an_otp() -> Result<String,StatusCode> {

    let otp = rand::thread_rng().gen_range(100000..999999);

    Ok(otp.to_string())
}
