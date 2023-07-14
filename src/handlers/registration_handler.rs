use std::sync::Arc;

use axum::{http::{StatusCode, Response, header}, response::IntoResponse, Json, extract::State};

use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(serde::Deserialize,serde::Serialize,Debug)]
pub struct Undergraduate {
    username: String,
    password: String,
    email: Option<String>,
    phone: Option<String>,
}

impl Undergraduate {

    fn hash_password(mut self) -> Self {
        self.password = hash_password(self.password).unwrap();
        self
    }

}

pub async fn register_an_undergraduate(
    State(db): State<Arc<Surreal<Client>>>,
    Json(new_user): Json<Undergraduate>
) -> Result<impl IntoResponse,StatusCode> {

    let new_user = new_user.hash_password();

    println!("{:?}", new_user);

    let undergraduate:Undergraduate = db
        .create("user")
        .content(new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    

    Ok("User created successfully")
}

fn hash_password(password: String) -> Result<String,StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}