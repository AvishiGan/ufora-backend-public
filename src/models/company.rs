use std::sync::Arc;
use simple_collection_macros::bmap;

use axum::http::StatusCode;
use surrealdb::{sql::{
    Thing, 
    statements::{CreateStatement, InsertStatement},
    Output,
    Fields,
    Field,
    Values,
    Value,
    Object,
    Strand,
    Table,
    Data,
    Idiom, Part, Ident
}, Surreal, engine::remote::ws::Client};



#[derive(serde::Serialize,serde::Deserialize)]
pub struct Company {
    id: Option<Thing>,
    name: Option<String>,
    email: Option<String>
}

impl Company {

    pub fn from(name: Option<String>, email: Option<String> ) -> Self {
        Self {
            id:None,
            name,
            email
        }
    }

    pub async fn get_register_query(
        self,
    ) -> Result<CreateStatement,StatusCode> {


        match (self.email.clone(),self.name.clone()) {
            (None,_) | (_,None) => Err(StatusCode::INTERNAL_SERVER_ERROR) ?,
            (_,_) => {}
        }

        // Ok(InsertStatement {
        //     into: Table("company".to_string()),
        //     data: Data::ContentExpression(Value::Object( Object (bmap! {
        //         "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
        //         "email".to_string() => Value::Strand(Strand(self.email.unwrap())),
        //     }))),
        //     ignore: false,
        //     update: None,
        //     output: Some(
        //         Output::Fields(
        //             Fields(vec![
        //                 Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident("id".to_string()))])))
        //                 ],true))
        //     ),timeout: None,
        //     parallel:false
        // })

        Ok(CreateStatement {
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
                        Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident("id".to_string()))])))
                        ],true))
            ),
            timeout: None,
            parallel: false,
        })

    }
}