use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal, sql::Thing};

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    first: &'static str,
    last: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    title: &'static str,
    name: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    success: &'static str,
}

pub async fn test_route(
    State(db): State<Arc<Surreal<Client>>>,
    // request: Request<Body>,
) -> (StatusCode, Json<TestResponse>) {
    // let claim = request
    //     .extensions()
    //     .get::<crate::models::user_claim::Claim>()
    //     .unwrap();

    let created: Result<Record, surrealdb::Error> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: "henry",
        })
        .await;

    // dbg!(created);

    //    ok json success
    // match created {
    //     Ok(_) => Ok(Json(Success { success: "success" })),
    //     Err(e) => {
    //         println!("{:?}", e);
    //         Err(StatusCode::INTERNAL_SERVER_ERROR)
    //     }
    // }

    match created {
        Ok(_) => (StatusCode::OK, Json(TestResponse { success: "Successfully inserted" })),
        Err(e) => {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(TestResponse { success: "Failed to insert" }))
        }        
    }

    // (StatusCode::OK, Json(TestResponse { success: "Successfully inserted" }))

}
