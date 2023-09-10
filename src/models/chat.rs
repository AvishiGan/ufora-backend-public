use std::sync::Arc;

use axum::Json;
use reqwest::StatusCode;
use simple_collection_macros::bmap;
use surrealdb::{
    engine::remote::ws::Client,
    sql::{
        statements::{CreateStatement, SelectStatement},
        Array, Cond, Data, Datetime, Expression, Field, Fields, Ident, Idiom, Object, Output, Part,
        Table, Thing, Value, Values,
    },
    Surreal,
};

use crate::services::query_builder::{get_select_query, Column, Item};

use super::user::User;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ChatMessage {
    id: Option<Thing>,
    from: Option<String>,
    message: Option<String>,
    image: Option<String>,
    time: Option<Datetime>,
    reply: Option<String>,

    // personal cat
    readflag: Option<bool>,

    // group chat
    announcement: Option<bool>,
    read_receipt: Option<Vec<ReadReceipt>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ReadReceipt {
    id: Option<Thing>,
    user: Option<String>,
    time: Option<Datetime>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct PersonalChat {
    id: Option<Thing>,
    messages: Option<Vec<ChatMessage>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct GroupChat {
    id: Option<Thing>,
    messages: Option<Vec<ChatMessage>>,
    admins: Option<Vec<String>>,
}

impl ChatMessage {
    pub fn new(
        from: Option<String>,
        message: Option<String>,
        image: Option<String>,
        time: Option<Datetime>,
        reply: Option<String>,
        readflag: Option<bool>,
        announcement: Option<bool>,
        read_receipt: Option<Vec<ReadReceipt>>,
    ) -> Self {
        Self {
            id: None,
            from,
            message,
            image,
            time,
            reply,
            readflag,
            announcement,
            read_receipt,
        }
    }

    // pub async fn personal_chat_message_query(&self) -> Result<String, StatusCode> {
    //     let stmt = format!("SELECT",{});
    //     Ok(stmt)
    // }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct People {
    chatmadeby: Option<String>,
    chatwith: Option<String>,
}

impl PersonalChat {
    pub fn new() -> Self {
        Self {
            id: None,
            messages: None,
        }
    }

    pub async fn create_chat_query(
        &self,
        claim: crate::models::user_claim::Claim,
        db: Arc<Surreal<Client>>,
        Json(people): Json<People>,
    ) -> Result<CreateStatement, StatusCode> {
        let chatmadewith =
            User::get_user_by_email_or_username(db.clone(), None, people.chatwith).await;

        let chatmadebyid = claim.get_surrealdb_thing();
        let chatmadewithid = chatmadewith.unwrap().get_id();

        let getifchat = SelectStatement {
            expr: Fields(vec![Field::All], true),
            what: Values(vec![Value::Table(Table("personalchat".to_string()))]),
            // omit: None,
            // only: false,
            // explain: false,
            // with: None,
            // check chatmadeby an chatmadewith
            cond: Some(Cond(Value::Array(Array(vec![
                Value::Expression(Box::from(Expression::Binary {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("chatmadeby".to_string()))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Thing(chatmadebyid.clone()),
                })),
                Value::Expression(Box::from(Expression::Binary {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("chatmadewith".to_string()))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Thing(chatmadewithid.clone()),
                })),
            ])))),

            //     Expression(Box::from(Expression {
            //     l: Value::Idiom(Idiom(vec![Part::Field(Ident("chatmadeby".to_string()))])),
            //     o: surrealdb::sql::Operator::Equal,
            //     r: Value::Thing(chatmadewithid.clone()),
            // }))
            group: None,
            order: None,
            limit: None,
            start: None,
            fetch: None,
            omit: None,
            only: false,
            explain: None,
            with: None,
            version: None,
            split: None,
            timeout: None,
            parallel: false,
        };

        println!("getifchat: {:?}", getifchat);

        let response = db.query(getifchat).await;

        match response {
            Ok(mut response) => {
                let chat: Result<Option<PersonalChat>, surrealdb::Error> = response.take(0);
                match chat {
                    Ok(Some(_)) => {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                    Ok(None) => {}
                    Err(_) => {
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            }
            Err(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }

        Ok(CreateStatement {
            what: Values(vec![Value::Table(Table("personalchat".to_string()))]),
            data: Some(Data::ContentExpression(Value::Object(Object(bmap!(
                "chatmadeby".to_string() => Value::Thing(chatmadebyid),
                "chatmadewith".to_string() => Value::Thing(chatmadewithid),

                "messages".to_string() => Value::Array(Array(vec![])),
            ))))),
            only: false,
            output: Some(Output::Null),
            timeout: None,
            parallel: false,
        })
    }

    pub async fn get_chats_by_user_id(
        db: Arc<Surreal<Client>>,
        user_id: Thing,
    ) -> Result<Vec<Self>, String> {
        let query = get_select_query(
            Item::Record {
                tb: user_id.tb,
                id: user_id.id.to_string(),
            },
            Column::Specific(vec!["->create_chat->personalchat   .* as chats".to_string()]),
            None,
            None,
            None,
            None,
            None,
        );

        let response = db.query(query).await;

        #[derive(serde::Deserialize, serde::Serialize, Debug)]
        struct Chats {
            chats: Vec<PersonalChat>,
        }

        match response {
            Ok(mut response) => {
                let chats: Result<Option<Chats>, surrealdb::Error> = response.take(0);
                match chats {
                    Ok(Some(chats)) => Ok(chats.chats),
                    Ok(None) => {
                        return Err("No chats found".to_string());
                    }
                    Err(e) => {
                        return Err(format!("{:?}", e.to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(format!("{:?}", e.to_string()));
            }
        }
    }
}
