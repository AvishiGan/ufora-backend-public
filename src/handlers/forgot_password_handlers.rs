use std::sync::Arc;

use axum::{ http::StatusCode, extract::State, Json };
use dotenvy_macro::dotenv;
use magic_crypt::{ MagicCryptTrait, new_magic_crypt };
use chrono::prelude::*;
use surrealdb::{ Surreal, engine::remote::ws::Client, opt::PatchOp, sql::{ Value, Strand } };

use crate::{ models::user::User, services::{ email, otp::{ self, OTP }, password } };

// structure for forgot password request
#[derive(serde::Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

// structure for forgot password response
#[derive(serde::Serialize)]
pub enum ForgotPasswordResponse {
    OTPSent {
        token: String,
    },
    InvalidEmail,
    InternalServerError,
}

// handler for sending otp to email
pub async fn verify_email_and_send_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(forgot_password_request): Json<ForgotPasswordRequest>
) -> (StatusCode, Json<ForgotPasswordResponse>) {
    // check whether email is empty or not
    if forgot_password_request.email.is_empty() {
        return (StatusCode::OK, Json(ForgotPasswordResponse::InvalidEmail));
    }

    // get user by email
    let user = User::get_user_by_email(db.clone(), forgot_password_request.email.clone()).await;

    // check whether user exists or not
    match user {
        Ok(user) => {
            // generate otp
            let otp = otp::get_an_otp();

            // check whether otp is generated or not
            match otp {
                Ok(_) => {}
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ForgotPasswordResponse::InternalServerError),
                    );
                }
            }

            // email body
            let email =
                "OTP to reset your password is ".to_string() +
                &otp.clone().unwrap() +
                ". Please do not share this OTP with anyone.";

            // send email
            let _response = email
                ::send_email(
                    &user.get_user_email(),
                    "OTP for reset password".to_string(),
                    email
                ).await
                .map_err(|_| {
                    return (StatusCode::OK, Json(ForgotPasswordResponse::InternalServerError));
                });

            // get current time from local timezone
            let utc = Utc.from_local_datetime(&chrono::Local::now().naive_local())
                .single()
                .unwrap();

            // update or insert otp in database
            let result: Option<OTP> = db
                .update(("otp", forgot_password_request.email.clone()))
                .merge(OTP {
                    otp: otp.unwrap(),
                    created_at: utc,
                    expires_at: utc + chrono::Duration::minutes(10),
                }).await
                .unwrap();

            // encrypt email as a token
            let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
            let encrypted_email = mcrypt.encrypt_str_to_base64(&forgot_password_request.email);

            match result {
                Some(_) =>
                    (
                        StatusCode::OK,
                        Json(ForgotPasswordResponse::OTPSent { token: encrypted_email }),
                    ),
                None =>
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ForgotPasswordResponse::InternalServerError),
                    ),
            }
        }

        // if user does not exist
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(ForgotPasswordResponse::InvalidEmail));
        }
    }
}

// request struct for verifying otp
#[derive(serde::Deserialize)]
pub struct OTPVerificationRequest {
    otp: String,
    token: String,
}

// response struct for verifying otp
#[derive(serde::Serialize)]
pub enum OTPVerificationResponse {
    OTPVerified {
        password_reset_token: String,
    },
    InvalidOTP {
        message: String,
    },
    InternalServerError,
}

// handler for verifying otp
pub async fn verify_forgot_password_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_verification_request): Json<OTPVerificationRequest>
) -> (StatusCode, Json<OTPVerificationResponse>) {
    // decrypt email from token
    let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
    let decrypted_email = mcrypt.decrypt_base64_to_string(&otp_verification_request.token);

    // check whether email is decrypted or not
    match decrypted_email {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(OTPVerificationResponse::InternalServerError),
            );
        }
    }

    // unwrap decrypted email
    let decrypted_email = decrypted_email.unwrap();

    // select otp from database
    let otp: Option<OTP> = db.select(("otp", decrypted_email.clone())).await.unwrap();

    match otp {
        Some(otp) => {
            // check whether otp has expired or not
            if
                otp.expires_at >
                Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap()
            {
                // check whether otp is valid or not
                if otp.otp == otp_verification_request.otp.clone() {
                    // delete otp from database
                    let _response: Option<OTP> = db
                        .delete(("otp", decrypted_email.clone())).await
                        .unwrap();

                    // get user by email
                    let user = User::get_user_by_email(db, decrypted_email.clone()).await.unwrap();

                    // encrypt user id as a token with the timestamp adding 10 minutes to it
                    let password_reset_token =
                        user.get_id().to_raw() +
                        "#" +
                        (
                            Utc.from_local_datetime(&chrono::Local::now().naive_local())
                                .single()
                                .unwrap() + chrono::Duration::minutes(10)
                        )
                            .to_string()
                            .as_str();
                    let encrypted_user_id = mcrypt.encrypt_str_to_base64(password_reset_token);
                    (
                        StatusCode::OK,
                        Json(OTPVerificationResponse::OTPVerified {
                            password_reset_token: encrypted_user_id,
                        }),
                    )
                } else {
                    // delete otp from database if otp is invalid
                    (
                        StatusCode::BAD_REQUEST,
                        Json(OTPVerificationResponse::InvalidOTP {
                            message: "OTP is invalid".to_string(),
                        }),
                    )
                }
            } else {
                // delete otp from database
                let _response: Option<OTP> = db
                    .delete(("otp", decrypted_email.clone())).await
                    .unwrap();
                (
                    StatusCode::BAD_REQUEST,
                    Json(OTPVerificationResponse::InvalidOTP {
                        message: "OTP has expired".to_string(),
                    }),
                )
            }
        }

        // if otp does not exist
        None =>
            (
                StatusCode::BAD_REQUEST,
                Json(OTPVerificationResponse::InvalidOTP {
                    message: "OTP is invalid".to_string(),
                }),
            ),
    }
}

// request struct for resetting password
#[derive(serde::Deserialize)]
pub struct ResetPasswordRequest {
    password: String,
    confirm_password: String,
    password_reset_token: String,
}

// response struct for resetting password
#[derive(serde::Serialize)]
pub enum ResetPasswordResponse {
    PasswordReset,
    InvalidInputs {
        message: String,
    },
    InternalServerError,
}

// handler for resetting password
pub async fn reset_password(
    State(db): State<Arc<Surreal<Client>>>,
    Json(reset_password_request): Json<ResetPasswordRequest>
) -> (StatusCode, Json<ResetPasswordResponse>) {
    // decrypt password reset token
    let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
    let decrypted_password_reset_token = mcrypt.decrypt_base64_to_string(
        &reset_password_request.password_reset_token
    );

    // check whether password reset token is decrypted or not
    match decrypted_password_reset_token {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResetPasswordResponse::InternalServerError),
            );
        }
    }

    // unwrap decrypted password reset token and take user id and token expiration time
    let decrypted_password_reset_token = decrypted_password_reset_token.unwrap();
    let decrypted_password_reset_token = decrypted_password_reset_token
        .split("#")
        .collect::<Vec<&str>>();

    // check whether user id and token expiration time are present or not
    if decrypted_password_reset_token.len() != 2 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResetPasswordResponse::InvalidInputs {
                message: "Invalid password reset token".to_string(),
            }),
        );
    }

    // unwrap user id and token expiration time
    let user_id = decrypted_password_reset_token[0];
    let token_expiration_time = decrypted_password_reset_token[1]
        .parse::<DateTime<Utc>>()
        .map_err(|_| Utc.from_local_datetime(&chrono::Local::now().naive_local()))
        .unwrap();

    // check whether token has expired or not
    if
        token_expiration_time >
        Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap()
    {
        // check whether password and confirm password are same or not
        if reset_password_request.password == reset_password_request.confirm_password {
            // hash password
            let hashed_password = password
                ::hash_password(reset_password_request.password.clone())
                .unwrap();

            // check whether password is hashed or not
            if hashed_password.is_empty() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResetPasswordResponse::InternalServerError),
                );
            }

            // temp structure to store password
            #[derive(serde::Serialize, serde::Deserialize)]
            struct TempPass {}

            // update password in database
            let _response: Vec<TempPass> = db
                .update(("user", user_id))
                .patch(PatchOp::replace("/password", Value::Strand(Strand(hashed_password)))).await
                .unwrap();
            (StatusCode::OK, Json(ResetPasswordResponse::PasswordReset))
        } else {
            // if password and confirm password are not same
            (
                StatusCode::BAD_REQUEST,
                Json(ResetPasswordResponse::InvalidInputs {
                    message: "Passwords do not match".to_string(),
                }),
            )
        }
    } else {
        // if token has expired
        (
            StatusCode::BAD_REQUEST,
            Json(ResetPasswordResponse::InvalidInputs {
                message: "Password reset token has expired".to_string(),
            }),
        )
    }
}
