use core::fmt;
use std::{str::FromStr, sync::Arc};

use chrono::prelude::*;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::services::query_builder::{
    get_create_query_for_an_object, get_delete_query_for_specific_record,
    get_relate_query_with_content, get_select_query, Column, DatabaseObject, Item, Return,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Post {
    id: Option<Thing>,
    caption: Option<String>,
    access_level: Option<AccessLevel>,
    content: Option<String>,
    reactions: Vec<Thing>,
    comments: Vec<Comment>,
    time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum AccessLevel {
    Public,
    Friends,
    OnlyMe,
}

impl FromStr for AccessLevel {
    type Err = ();

    fn from_str(input: &str) -> Result<AccessLevel, Self::Err> {
        match input {
            "public" => Ok(AccessLevel::Public),
            "friends" => Ok(AccessLevel::Friends),
            "only me" => Ok(AccessLevel::OnlyMe),
            _ => Err(()),
        }
    }
}

impl fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccessLevel::Public => write!(f, "public"),
            AccessLevel::Friends => write!(f, "friends"),
            AccessLevel::OnlyMe => write!(f, "only me"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Comment {
    #[serde(default)]
    pub id: String,
    pub reply: Option<String>,
    pub user: Thing,
    pub text: String,
    pub time: String,
}

impl Post {
    pub fn new(
        caption: Option<String>,
        access_level: Option<AccessLevel>,
        content: Option<String>,
    ) -> Self {
        Self {
            id: None,
            caption,
            access_level,
            content,
            reactions: vec![],
            comments: vec![],
            time: chrono::Utc
                .from_local_datetime(&chrono::Local::now().naive_local())
                .single()
                .unwrap()
                .timestamp()
                .to_string(),
        }
    }

    pub async fn save(&self, db: Arc<Surreal<Client>>, user_id: Thing) -> Result<(), String> {
        match (
            self.caption.clone(),
            self.access_level.clone(),
            self.content.clone(),
        ) {
            (None, _, _) | (_, None, _) | (_, _, None) => {
                return Err("caption, access_level and content are required".to_string());
            }
            (_, _, _) => {}
        }

        let post_create_query = get_create_query_for_an_object(
            Item::Table("post".to_string()),
            DatabaseObject {
                keys: vec![
                    "caption".to_string(),
                    "access_level".to_string(),
                    "content".to_string(),
                    "reactions".to_string(),
                    "comments".to_string(),
                    "delete".to_string(),
                    "time".to_string(),
                ],
                values: vec![
                    "'".to_string() + &self.caption.clone().unwrap() + "'",
                    "'".to_string() + &self.access_level.clone().unwrap().to_string() + "'",
                    "'".to_string() + &self.content.clone().unwrap() + "'",
                    "[]".to_string(),
                    "[]".to_string(),
                    "None".to_string(),
                    "'".to_string() + self.time.clone().as_ref() + "'",
                ],
            },
            Return::Fields {
                fields: vec!["id".to_string()],
            },
        );

        let response = db.query(post_create_query).await;

        #[derive(serde::Deserialize, Debug)]
        struct RecordID {
            id: Thing,
        }

        let post_id = match response {
            Ok(mut response) => {
                let record_id: Result<Option<RecordID>, surrealdb::Error> = response.take(0);
                match record_id {
                    Ok(Some(record_id)) => record_id.id,
                    Ok(None) => {
                        return Err("Post could not be created".to_string());
                    }
                    Err(e) => {
                        return Err(format!("{:?}", e.to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(format!("{:?}", e.to_string()));
            }
        };

        self.link_user_with_post(db.clone(), user_id, post_id).await
    }

    async fn link_user_with_post(
        &self,
        db: Arc<Surreal<Client>>,
        user_id: Thing,
        post_id: Thing,
    ) -> Result<(), String> {
        let relate_query =
            get_relate_query_with_content(user_id, post_id, "create_post".to_string(), None);

        let response = db.query(relate_query).await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e.to_string())),
        }
    }

    pub async fn get_post_by_user_id(
        db: Arc<Surreal<Client>>,
        user_id: Thing,
    ) -> Result<Vec<Self>, String> {
        let query = get_select_query(
            Item::Record {
                tb: user_id.tb,
                id: user_id.id.to_string(),
            },
            Column::Specific(vec!["->create_post->post.* as posts".to_string()]),
            None,
            None,
            None,
            None,
            None,
        );

        let response = db.query(query).await;

        #[derive(serde::Deserialize, Debug)]
        struct Posts {
            pub posts: Vec<Post>,
        }

        match response {
            Ok(mut response) => {
                let posts: Result<Option<Posts>, surrealdb::Error> = response.take(0);
                match posts {
                    Ok(posts) => Ok(posts.unwrap().posts),
                    Err(e) => Err(format!("{:?}", e.to_string())),
                }
            }
            Err(e) => Err(format!("{:?}", e.to_string())),
        }
    }

    pub async fn delete_post_by_id(
        db: Arc<Surreal<Client>>,
        post_id: String,
    ) -> Result<(), String> {
        let delete_query = get_delete_query_for_specific_record("post".to_string(), post_id);

        let response = db.query(delete_query).await;

        match response {
            Ok(mut response) => {
                let post: Result<Vec<Post>, surrealdb::Error> = response.take(0);
                if post.unwrap().len() == 0 {
                    return Err("Post not found".to_string());
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("{:?}", e.to_string())),
        }
    }

    pub async fn add_or_remove_reaction(
        db: Arc<Surreal<Client>>,
        post_id: Thing,
        user_id: Thing,
    ) -> Result<(), String> {
        let query = "LET $reactions = (SELECT VALUE reactions FROM ".to_string()
            + &post_id.to_string()
            + " ); "
            // Check whether user has already reacted to the post or not
            + "IF $reactions CONTAINS "
            + &user_id.to_string()
            // If user has already reacted, then remove the reaction
            + " THEN (UPDATE "
            + &post_id.to_string()
            + " SET reactions -= [ "
            + &user_id.to_string()
            + " ])"
            // If user has not reacted, then add the reaction
            + " ELSE (UPDATE "
            + &post_id.to_string()
            + " SET reactions += [ "
            + &user_id.to_string()
            + " ])"
            + " END;";

        let response = db.query(query).await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e.to_string())),
        }
    }

    pub async fn add_a_comment(
        db: Arc<Surreal<Client>>,
        post_id: Thing,
        comment : Comment,
    ) -> Result<(),String> {
        let comment_json_string = serde_json::to_string(&comment).unwrap();

        let query = "UPDATE ".to_string()
            + &post_id.to_string()
            + " SET comments += [ "
            + &comment_json_string
            + " ]";

        let response = db.query(query).await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e.to_string())),
        }
    }
}
