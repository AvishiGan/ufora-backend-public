
use std::{sync::Arc, vec};

use axum::http::StatusCode;
use simple_collection_macros::bmap;
use surrealdb::{sql::{Thing, statements::{CreateStatement, SelectStatement, UpdateStatement}, Values, Value, Table, Data, Object, Strand, Output, Number, Fields, Limit, Cond, Expression, Part, Ident, Idiom, Field, Operator}, Surreal, engine::remote::ws::Client, opt::PatchOp};

use crate::services::password;

// model for user
#[derive(serde::Serialize,serde::Deserialize,Default,Debug)]
pub struct User {
    id: Option<Thing>,
    username: Option<String>,
    password: Option<String>,
    locked_flag: Option<bool>,
    user_type: Option<String>,
    user_id: Option<Thing>,
    email:Option<String>,
    email_verification_flag: Option<bool>,
    pub invalid_login_attempts: Option<i32>,
}

impl User {

    // returns a new user model
    pub fn from(username: Option<String>, password: Option<String>,email:Option<String>) -> Self {
        Self {
            id: None,
            username,
            password,
            email,
            ..Default::default()
        }
    }

    // returns the surrealQl query for creating a user
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
                "email".to_string() => Value::Strand(Strand(self.email.unwrap())),
                "email_verification_flag".to_string() => Value::False,
                "invalid_login_attempts".to_string() => Value::Number(Number::Int(0))
            ))))),
            output: Some(Output::Null),
            timeout: None,
            parallel: false
        })

    }
    
    // returns the user from the database
    pub async fn retrieve_user_from_database_by_username(
        db:Arc<Surreal<Client>>,username: String
    ) -> Result<Self,StatusCode> {

        let mut response = db.query(SelectStatement {
            expr: Fields (
                vec![Field::All],
                true
            ),
            what: Values(
                vec![Value::Table(Table("user".to_string()))]
            ),
            cond:Some(Cond(
                Value::Expression(Box::from(Expression {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("username".to_string()))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Strand(Strand(username))
                })))
            ),
            group: None,
            order: None,
            limit: Some(Limit(Value::Number(Number::Int(1)))),
            start: None,
            fetch: None,
            version: None,
            split:None,
            timeout:None,
            parallel:false

        }).await.unwrap();

        let users: Option<Self> = response.take(0).unwrap();

        match users {
            Some(user) => Ok(user),
            None => Err(StatusCode::NOT_FOUND)
        }

    }

    // gets the stored password
    pub fn get_password(&self) -> Option<String> {
        self.password.clone()
    }

    // updates the invalid login attempts and locked account
    pub async fn update_login_attempts(
        self,
        db:Arc<Surreal<Client>>,
        new_invalid_login_attempts: i32
    ) -> () {

        #[derive(serde::Deserialize)]
        struct LoginAttemptUpdateResult {}

        let _response: Option<LoginAttemptUpdateResult> = match new_invalid_login_attempts  {
            0..=4 => {
                db.update(("user",self.id.unwrap().id))
                    .patch(PatchOp::replace("/invalid_login_attempts",new_invalid_login_attempts))
                    .await.unwrap_or(None)
            },
            5 => {
                db.update(("user",self.id.unwrap().id))
                    .patch(PatchOp::replace("/invalid_login_attempts",new_invalid_login_attempts))
                    .patch(PatchOp::replace("/locked_flag",true))
                    .await.unwrap_or(None)
            }
            _ => {
                None
            }
        };
        
    }

    // returns whether the user is locked or not
    pub fn is_user_locked(&self) -> bool {
        self.locked_flag.unwrap()
    }

    // returns user type
    pub fn get_user_type(&self) -> String {
        self.user_type.as_ref().unwrap().clone()
    }

    // returns user id
    pub fn get_user_id(&self) -> Thing {
        self.user_id.as_ref().unwrap().clone()
    }

    // returns whether the user is verified or not
    pub async fn update_email_verification(
        db:Arc<Surreal<Client>>,
        email: String
    ) -> Result<(),StatusCode> {

        let _response = db.query(
            UpdateStatement {
                what: Values(
                    vec![Value::Table(Table("user".to_string()))]
                ),
                data: Some(Data::SetExpression(
                    vec![
                        (
                            Idiom(vec![Part::Field(Ident("email_verification_flag".to_string()))]),
                            Operator::Equal,
                            Value::True
                        )
                    ]
                )),
                cond: Some(Cond(
                    Value::Expression(Box::from(Expression {
                        l: Value::Idiom(Idiom(vec![Part::Field(Ident("email".to_string()))])),
                        o: surrealdb::sql::Operator::Equal,
                        r: Value::Strand(Strand(email))
                    })))
                ),
                output: None,
                timeout: None,
                parallel: false
            }
        ).await;

        Ok(())
    }

    // returns user by email
    pub async fn get_user_by_email(
        db:Arc<Surreal<Client>>,
        email: String
    ) -> Result<Self,StatusCode> {

        let mut response = db.query(SelectStatement {
            expr: Fields (
                vec![Field::All],
                true
            ),
            what: Values(
                vec![Value::Table(Table("user".to_string()))]
            ),
            cond:Some(Cond(
                Value::Expression(Box::from(Expression {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("email".to_string()))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Strand(Strand(email))
                })))
            ),
            group: None,
            order: None,
            limit: Some(Limit(Value::Number(Number::Int(1)))),
            start: None,
            fetch: None,
            version: None,
            split:None,
            timeout:None,
            parallel:false

        }).await.unwrap();

        let users: Option<Self> = response.take(0).unwrap();

        match users {
            Some(user) => Ok(user),
            None => Err(StatusCode::NOT_FOUND)
        }

    }

    pub fn get_user_email(&self) -> String {
        match self.email.clone() {
            Some(email) => email,
            None => "".to_string()
        }
    }

}