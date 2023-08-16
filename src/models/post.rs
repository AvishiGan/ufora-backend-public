use std::{ sync::Arc, fmt::format };

use surrealdb::{ Surreal, engine::remote::ws::Client, sql::Thing };
use chrono::prelude::*;

use crate::services::queryBuilder::{
    Item,
    Return,
    DatabaseObject,
    get_create_query_for_an_object,
    get_relate_query_with_content, get_select_query, Column,
};

use super::user;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Post {
    id: Option<Thing>,
    caption: Option<String>,
    access_level: Option<String>,
    content: Option<String>,
    reactions: Vec<Thing>,
    comments: Vec<Comment>,
    delete: Option<String>,
    time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Comment {
    id: String,
    reply: Option<String>,
    user: Thing,
    text: String,
    time: String,
}

impl Post {
    pub fn new(
        caption: Option<String>,
        access_level: Option<String>,
        content: Option<String>
    ) -> Self {
        Self {
            id: None,
            caption,
            access_level,
            content,
            reactions: vec![],
            comments: vec![],
            delete: None,
            time: chrono::Utc
                .from_local_datetime(&chrono::Local::now().naive_local())
                .single()
                .unwrap()
                .timestamp()
                .to_string(),
        }
    }

    pub async fn save(&self, db: Arc<Surreal<Client>>, user_id: Thing) -> Result<(), String> {
        match (self.caption.clone(), self.access_level.clone(), self.content.clone()) {
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
                    "time".to_string()
                ],
                values: vec![
                    "'".to_string() + &self.caption.clone().unwrap() + "'",
                    "'".to_string() + &self.access_level.clone().unwrap() + "'",
                    "'".to_string() + &self.content.clone().unwrap() + "'",
                    "[]".to_string(),
                    "[]".to_string(),
                    "None".to_string(),
                    "'".to_string() + self.time.clone().as_ref() + "'"
                ],
            },
            Return::Fields { fields: vec!["id".to_string()] }
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
        post_id: Thing
    ) -> Result<(), String> {
        let relate_query = get_relate_query_with_content(
            user_id,
            post_id,
            "create_post".to_string(),
            None
        );

        let response = db.query(relate_query).await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e.to_string())),
        }
    }

    pub async fn get_post_by_user_id(
        db: Arc<Surreal<Client>>,
        user_id: Thing
    ) -> Result<Vec<Self>,String> {

        let query = get_select_query(Item::Record { tb: user_id.tb, id: user_id.id.to_string() }, Column::Specific(vec!["->create_post->post.* as posts".to_string()]), None, None, None, None, None);

        let response = db.query(query).await;

        #[derive(serde::Deserialize, Debug)]
        struct Posts {
            pub posts: Vec<Post>
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

}
