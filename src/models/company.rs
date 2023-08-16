use dotenvy_macro::dotenv;
use std::sync::Arc;

use simple_collection_macros::bmap;

use axum::http::StatusCode;
use surrealdb::{
    engine::remote::ws::Client,
    opt::PatchOp,
    sql::{
        statements::CreateStatement, Data, Field, Fields, Ident, Idiom, Object, Output, Part,
        Strand, Table, Thing, Value, Values,
    },
    Surreal,
};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Company {
    id: Option<Thing>,
    name: Option<String>,
    address: Option<String>,
    gmap: Option<String>,
}

impl Company {
    pub fn from(name: Option<String>, address: Option<String>, gmap: Option<String>) -> Self {
        Self {
            id: None,
            name,
            address,
            gmap,
        }
    }

    pub async fn get_register_query(self) -> Result<CreateStatement, StatusCode> {
        match self.name.clone() {
            None => Err(StatusCode::BAD_REQUEST)?,
            _ => {}
        }

        Ok(CreateStatement {
            what: Values(vec![Value::Table(Table("company".to_string()))]),
            data: Some(Data::ContentExpression(Value::Object(Object(bmap! {
                "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
            })))),
            output: Some(Output::Fields(Fields(
                vec![Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident(
                    "id".to_string(),
                ))])))],
                true,
            ))),
            timeout: None,
            parallel: false,
        })
    }
 
}

pub async fn update_address(
    user_id: String,
    db: Arc<Surreal<Client>>,
    address: Option<String>,
) -> Result<(), StatusCode> {
    match address {
        None => return Err(StatusCode::BAD_REQUEST),
        _ => {}
    }

    let gmap = format!(
        "https://www.google.com/maps/embed/v1/place?key={}&q={}",
        dotenv!("MAP_API_KEY"),
        address.clone().unwrap()
    )
    .replace(" ", "%20");

    let response: Result<String, surrealdb::Error> = db
        .update(("company", user_id))
        .patch(PatchOp::replace("/address", address.unwrap().to_string()))
        .patch(PatchOp::replace("/gmap", gmap.to_string()))
        .await;

    match response {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?} ", e);
            Ok(())
        }
    }
}
 

impl Into<serde_json::Value> for Company {
    fn into(self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "address": self.address,
            "gmap": self.gmap,
        })
    }
}