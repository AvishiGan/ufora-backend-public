use std::sync::Arc;

use axum::{
    Router,
    routing::get, http::StatusCode, Json, extract::State
};
use surrealdb::{Surreal, engine::remote::ws::Client};



pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/test", get(test_handler))
}

async fn test_handler(
    State(db): State<Arc<Surreal<Client>>>,
    Json(new_company): Json<crate::models::company::Company>
) -> Result<String,StatusCode> {
    let result = new_company.register_a_company(db).await?;
    println!("{:?}",result);
    Ok("Fit".to_owned())
}