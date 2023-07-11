mod app;
mod routes;
mod database;
mod middlewares;
mod errors;

#[tokio::main]
async fn main() {
    
    app::run().await.unwrap();


}
