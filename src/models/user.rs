use std::sync::Arc;

use axum::http::StatusCode;
use simple_collection_macros::bmap;
use surrealdb::{sql::{Thing, statements::{CreateStatement, SelectStatement}, Values, Value, Table, Data, Object, Strand, Output, Param, Ident, Subquery, Fields, Field, Idiom, Part, Limit}, Surreal, engine::remote::ws::Client};

use crate::services::password;

#[derive(serde::Serialize,serde::Deserialize,Default)]
pub struct User {
    id: Option<Thing>,
    username: Option<String>,
    password: Option<String>,
    locked_flag: Option<bool>,
    user_type: Option<String>,
    user_id: Option<Thing>
}

impl User {

    pub fn from(username: Option<String>, password: Option<String>) -> Self {
        Self {
            id: None,
            username,
            password,
            ..Default::default()
        }
    }

    pub async fn get_create_user_query(
        self,
        user_type: String,
        user_id: Option<Thing>
    ) -> Result<CreateStatement,StatusCode> {

        match (self.username.clone(),self.password.clone(),user_id.clone()) {
            (None,_,_) | (_, None,_) | (_,_,None) => Err(StatusCode::INTERNAL_SERVER_ERROR) ?,
            (_,_,_) => {}
        }

        Ok(CreateStatement {
            what: Values(
                vec![Value::Table(Table("user".to_string()))]
            ),
            data: Some(Data::ContentExpression(Value::Object(Object(bmap!(
                "username".to_string() => Value::Strand(Strand(self.username.unwrap())),
                "password".to_string() => Value::Strand(Strand(password::hash_password(self.password.unwrap())?)),
                "locked_flag".to_string() => Value::False,
                "user_type".to_string() => Value::Strand(Strand(user_type)),
                "user_id".to_string() => Value::Thing(Thing::from(user_id.unwrap())),
            ))))),
            output: Some(Output::Null),
            timeout: None,
            parallel: false
        })

    }
    
}