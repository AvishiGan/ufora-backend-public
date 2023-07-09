mod app;
mod routes;
mod database;
mod middlewares;

#[tokio::main]
async fn main() {
    
    app::run().await.unwrap();


}
