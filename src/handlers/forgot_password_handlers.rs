use std::sync::Arc;

use axum::{http::StatusCode, extract::State, Json};
use dotenvy_macro::dotenv;
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use chrono::prelude::*;
use surrealdb::{Surreal, engine::remote::ws::Client, opt::PatchOp, sql::{Value, Strand}};

use crate::{models::user::User, services::{email, otp::{self, OTP}, password}};

// structure for forgot password request
#[derive(serde::Deserialize)]
pub struct ForgotPasswordRequest {
    pub email:String
}

// structure for forgot password response
#[derive(serde::Serialize)]
pub enum ForgotPasswordResponse {
    OTPSent {token:String},
    InvalidEmail,
    InternalServerError
}

pub async fn verify_email_and_send_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(forgot_password_request): Json<ForgotPasswordRequest>
) -> (StatusCode,Json<ForgotPasswordResponse>) {

    if forgot_password_request.email.is_empty() {
        return (StatusCode::OK,Json(ForgotPasswordResponse::InvalidEmail))
    }

    let user = User::get_user_by_email(db.clone(),forgot_password_request.email.clone()).await;

    match user {
        Ok(user) => {

            let otp = otp::get_an_otp();

            match otp {
                Ok(_) => {},
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR,Json(ForgotPasswordResponse::InternalServerError))
                }
            }

            let email = "OTP to reset your password is ".to_string() + &otp.clone().unwrap() + ". Please do not share this OTP with anyone.";
            
            let _response = email::send_email(&user.get_user_email(), "OTP for reset password".to_string(), email).await.map_err(|_| {
                return (StatusCode::OK,Json(ForgotPasswordResponse::InternalServerError))
            });

            let utc = Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap();

            let result: Option<OTP> = db.update(("otp",forgot_password_request.email.clone())).merge(OTP {
                otp: otp.unwrap(),
                created_at:utc,
                expires_at:utc + chrono::Duration::minutes(10)
                }).await.unwrap();

            let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
            let encrypted_email = mcrypt.encrypt_str_to_base64(&forgot_password_request.email);

            match result {
                Some(_) => (StatusCode::OK,Json(ForgotPasswordResponse::OTPSent {token:encrypted_email})),
                None => (StatusCode::INTERNAL_SERVER_ERROR,Json(ForgotPasswordResponse::InternalServerError))
            }

        },
        Err(_) => {
            return (StatusCode::OK,Json(ForgotPasswordResponse::InvalidEmail))
        }   
    }

}

// request struct for verifying otp
#[derive(serde::Deserialize)]
pub struct OTPVerificationRequest {
    otp:String,
    token:String
}

// response struct for verifying otp
#[derive(serde::Serialize)]
pub enum OTPVerificationResponse {
    OTPVerified {password_reset_token:String},
    InvalidOTP {message:String},
    InternalServerError
}

// handler for verifying otp
pub async fn verify_forgot_password_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_verification_request): Json<OTPVerificationRequest>
) -> (StatusCode,Json<OTPVerificationResponse>) {

    let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
    let decrypted_email = mcrypt.decrypt_base64_to_string(&otp_verification_request.token);

    match decrypted_email {
        Ok(_) => {},
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(OTPVerificationResponse::InternalServerError))
        }
    }

    let decrypted_email = decrypted_email.unwrap();

    let otp:Option<OTP> = db.select(("otp",decrypted_email.clone())).await.unwrap(); 

    match otp {
        Some(otp) => {
            if otp.expires_at > Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap() {
                if otp.otp == otp_verification_request.otp.clone() {
                    let _response:Option<OTP> = db.delete(("otp",decrypted_email.clone())).await.unwrap();
                    let user = User::get_user_by_email(db, decrypted_email.clone()).await.unwrap();
                    let password_reset_token = user.get_user_id().to_raw() + "#" + (Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap() + chrono::Duration::minutes(10) ).to_string().as_str();
                    let encrypted_user_id = mcrypt.encrypt_str_to_base64(password_reset_token);
                    (StatusCode::OK,Json(OTPVerificationResponse::OTPVerified {
                        password_reset_token:encrypted_user_id
                    }))
                } else {
                    (StatusCode::BAD_REQUEST,Json(OTPVerificationResponse::InvalidOTP {
                        message:"OTP is invalid".to_string()
                    }))
                }
            } else {
                let _response:Option<OTP> = db.delete(("otp",decrypted_email.clone())).await.unwrap();
                (StatusCode::BAD_REQUEST,Json(OTPVerificationResponse::InvalidOTP {
                    message:"OTP has expired".to_string()
                }))
            }
        },
        None => (StatusCode::BAD_REQUEST,Json(OTPVerificationResponse::InvalidOTP {
            message:"OTP is invalid".to_string()
        }))
    }

}

// request struct for resetting password
#[derive(serde::Deserialize)]
pub struct ResetPasswordRequest {
    password:String,
    confirm_password:String,
    password_reset_token:String
}

// response struct for resetting password
#[derive(serde::Serialize)]
pub enum ResetPasswordResponse {
    PasswordReset,
    InvalidInputs {
        message:String
    },
    InternalServerError
}

// handler for resetting password
pub async fn reset_password(
    State(db): State<Arc<Surreal<Client>>>,
    Json(reset_password_request): Json<ResetPasswordRequest>
) -> (StatusCode,Json<ResetPasswordResponse>) {

    let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
    let decrypted_password_reset_token = mcrypt.decrypt_base64_to_string(&reset_password_request.password_reset_token);

    match decrypted_password_reset_token {
        Ok(_) => {},
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(ResetPasswordResponse::InternalServerError))
        }
    }

    let decrypted_password_reset_token = decrypted_password_reset_token.unwrap();
    let decrypted_password_reset_token = decrypted_password_reset_token.split("#").collect::<Vec<&str>>();

    if decrypted_password_reset_token.len() != 2 {
        return (StatusCode::BAD_REQUEST,Json(ResetPasswordResponse::InvalidInputs {
            message:"Invalid password reset token".to_string()
        }))
    }

    let user_id = decrypted_password_reset_token[0];
    let token_expiration_time = decrypted_password_reset_token[1].parse::<DateTime<Utc>>().map_err(|_| Utc.from_local_datetime(&chrono::Local::now().naive_local())).unwrap();

    if token_expiration_time > Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap() {
        if reset_password_request.password == reset_password_request.confirm_password {
            let hashed_password = password::hash_password(reset_password_request.password.clone()).unwrap();

            if hashed_password.is_empty() {
                return (StatusCode::INTERNAL_SERVER_ERROR,Json(ResetPasswordResponse::InternalServerError))
            }

            // temp structure to store password
            #[derive(serde::Serialize,serde::Deserialize)]
            struct TempPass {}

            let _response:Vec<TempPass> =db.update(("user",user_id)).patch(
                PatchOp::replace(
                    "/password",
                    Value::Strand(Strand(hashed_password))
                )
            ).await.unwrap();
            (StatusCode::OK,Json(ResetPasswordResponse::PasswordReset))
        }
        else {
            (StatusCode::BAD_REQUEST,Json(ResetPasswordResponse::InvalidInputs {
                message:"Passwords do not match".to_string()
            }))
        }
    }
    else {
        (StatusCode::BAD_REQUEST,Json(ResetPasswordResponse::InvalidInputs {
            message:"Password reset token has expired".to_string()
        }))
    }
    
}