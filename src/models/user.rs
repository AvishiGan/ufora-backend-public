
use axum::http::StatusCode;
use simple_collection_macros::bmap;
use surrealdb::sql::{Thing, statements::CreateStatement, Values, Value, Table, Data, Object, Strand, Output, Number};

use crate::services::password;

#[derive(serde::Serialize,serde::Deserialize,Default)]
pub struct User {
    id: Option<Thing>,
    username: Option<String>,
    password: Option<String>,
    locked_flag: Option<bool>,
    user_type: Option<String>,
    user_id: Option<Thing>,
    invalid_login_attempts: Option<i32>,
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
            (None,_,_) | (_, None,_) | (_,_,None) => Err(StatusCode::BAD_REQUEST) ?,
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
                "invalid_login_attempts".to_string() => Value::Number(Number::Int(0))
            ))))),
            output: Some(Output::Null),
            timeout: None,
            parallel: false
        })

    }
    
}