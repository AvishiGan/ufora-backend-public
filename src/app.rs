
use crate::routes;

pub async fn run() -> Result<(), String> {

    let app = routes::get_router();

    axum::Server::bind(&"0.0.0.0.3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())


}