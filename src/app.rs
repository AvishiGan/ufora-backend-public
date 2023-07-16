
use crate::routes;
use crate::database;

use axum::Router;
use dotenvy_macro::dotenv;

pub async fn run() -> Result<(), String> {

    

    let databsase_credentials = (
        dotenv!("DB_URI"),
        dotenv!("DB_NS"),
        dotenv!("DB_NAME"),
        dotenv!("DB_USERNAME"),
        dotenv!("DB_PASSWORD")
    );

    let db = database::connect(databsase_credentials).await?;

    let app:Router = routes::get_router()
    .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())


}