
use std::sync::Arc;

use axum::{http::StatusCode, extract::State, Json};
use chrono::prelude::*;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::{services::{otp::{self, OTP},email}, models::{user::User, undergraduate::Undergraduate}};

// request struct for sending otp to email
#[derive(serde::Deserialize)]
pub struct OTPRequest {
    email:String
}

// response struct for sending otp to email
#[derive(serde::Serialize)]
pub struct OTPSendingResponse {
    message:String,
}

// handler for sending otp to email
pub async fn send_otp_to_email(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_request): Json<OTPRequest>
) -> Result<Json<OTPSendingResponse>,StatusCode> {

    let otp = otp::get_an_otp().unwrap();

    let email = "OTP for your email verification is ".to_string() + &otp + ". Please do not share this OTP with anyone.";

    email::send_email(("Receiver <".to_string() + &otp_request.email + ">").as_ref(), "OTP for your registration".to_string(), email).await?;

    let utc = Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap();

    let result: Option<OTP> = db.update(("otp",otp_request.email)).merge(OTP {
        otp,
        created_at:utc,
        expires_at:utc + chrono::Duration::minutes(10)
    }).await.unwrap();

    match result {
        Some(_) => Ok(Json(OTPSendingResponse {
            message:"OTP has been sent to your email".to_string()
        })),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}

// request struct for verifying otp
#[derive(serde::Deserialize)]
pub struct OTPVerificationRequest {
    otp:String,
    email:String
}

// response struct for verifying otp
#[derive(serde::Serialize)]
pub struct OTPVerificationResponse {
    message:String,
}

// handler for verifying otp
pub async fn verify_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_verification_request): Json<OTPVerificationRequest>
) -> Result<Json<OTPVerificationResponse>,StatusCode> {

    let result: Option<OTP> = db.select(("otp",otp_verification_request.email.clone())).await.unwrap();

    match result {
        Some(otp) => {
            if otp.expires_at > Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap() {
                if otp.otp == otp_verification_request.otp.clone() {
                    let _response:Option<OTP> = db.delete(("otp",otp_verification_request.email.clone())).await.unwrap();
                    User::update_email_verification(db.clone(), otp_verification_request.email.clone()).await.unwrap();
                    Ok(Json(OTPVerificationResponse {
                        message:"OTP has been verified successfully".to_string()
                    }))
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            } else {
                let _response:Option<OTP> = db.delete(("otp",otp_verification_request.email)).await.unwrap();
                Err(StatusCode::BAD_REQUEST)
            }
        },
        None => Err(StatusCode::BAD_REQUEST)
    }

}

pub async fn verify_otp_university_email(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_verification_request): Json<OTPVerificationRequest>
) -> Result<Json<OTPVerificationResponse>,StatusCode> {

    let result: Option<OTP> = db.select(("otp",otp_verification_request.email.clone())).await.unwrap();

    match result {
        Some(otp) => {
            if otp.expires_at > Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap() {
                if otp.otp == otp_verification_request.otp.clone() {
                    let _response:Option<OTP> = db.delete(("otp",otp_verification_request.email.clone())).await.unwrap();
                    Undergraduate::update_university_email_verification(db.clone(), otp_verification_request.email.clone()).await.unwrap();
                    Ok(Json(OTPVerificationResponse {
                        message:"OTP has been verified successfully".to_string()
                    }))
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            } else {
                let _response:Option<OTP> = db.delete(("otp",otp_verification_request.email)).await.unwrap();
                Err(StatusCode::BAD_REQUEST)
            }
        },
        None => Err(StatusCode::BAD_REQUEST)
    }
}