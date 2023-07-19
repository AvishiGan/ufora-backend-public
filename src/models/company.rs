use std::sync::Arc;
use simple_collection_macros::bmap;

use axum::http::StatusCode;
use surrealdb::{sql::{
    Thing, 
    statements::CreateStatement,
    Output,
    Fields,
    Field,
    Values,
    Value,
    Object,
    Strand,
    Table,
    Data
}, Surreal, engine::remote::ws::Client};



#[derive(serde::Serialize,serde::Deserialize)]
pub struct Company {
    id: Option<Thing>,
    name: Option<String>,
    email: Option<String>
}

impl Company {

    pub async fn register_a_company(self,db: Arc<Surreal<Client>>) -> Result<(Thing),StatusCode> {


        match (self.email.clone(),self.name.clone()) {
            (None,_) | (_,None) => Err(StatusCode::INTERNAL_SERVER_ERROR) ?,
            (_,_) => {}
        }

        let create_statement = CreateStatement {
            what: Values(
                vec![Value::Table(Table("company".to_string()))]
            ),
            data: Some(Data::ContentExpression(Value::Object( Object (bmap! {
                "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
                "email".to_string() => Value::Strand(Strand(self.email.unwrap())),
            })))),
            output: Some(
                Output::Fields(
                    Fields(vec![
                        Field::Alone(Value::Strand(Strand("id".to_string())))
                        ],true))
            ),
            timeout: None,
            parallel: false,
        }; 

        let result = db.query(create_statement.to_string()).await;
        
        match result {

            Ok(mut result) => {
                let new_id: Option<String> = result.take(0).unwrap();
            
                match new_id {
                    Some(id) => Ok(Thing::from(("company",id.as_ref()))),
                    None => Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
        } 

    }
}