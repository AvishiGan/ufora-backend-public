use axum::http::StatusCode;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use dotenvy_macro::dotenv;

// function to send email
pub async fn send_email(
    receiver_email: &str, // should be in the format "Receiver <user_email>"
    subject: String,
    body: String,
) -> Result<(), StatusCode> {
    // create email
    let email = Message::builder()
        .from(
            ("Ufora <".to_string() + dotenv!("APP_EMAIL") + ">")
                .parse()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
        .to(receiver_email.parse().map_err(|e| {
            println!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?)
        .subject(subject)
        .body(body)
        .unwrap();

    // email credentials for gmail
    let credentials = Credentials::new(
        dotenv!("APP_EMAIL").to_string(),
        dotenv!("APP_EMAIL_PASSWORD").to_string(),
    );

    // create mailer
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(credentials)
            .build();

    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
