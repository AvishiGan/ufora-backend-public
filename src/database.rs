use std::sync::Arc;

use axum::response::IntoResponse;
use surrealdb::{
    Surreal, engine::remote::ws::{Client, Ws},
    opt::auth::Root
};


pub async fn connect(
    (
        db_uri,
        db_ns,
        db_name,
        username,
        password
    ) : ( &str, &str, &str, &str, &str)
) -> Result<Arc<Surreal<Client>>,String> {

    let db = Surreal::new::<Ws>(db_uri)
    .await
    .map_err(|e| {println!("{:?}",e);e.to_string()})?;

    db.signin( Root {
        username,
        password
    })
    .await
    .map_err(|e| e.to_string())?;

    db.use_ns(db_ns).use_db(db_name)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Arc::new(db))
}