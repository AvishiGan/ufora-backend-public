use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use validator::Validate;

use crate::services::query_builder::get_relate_query_with_content;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Blog {
    id: Option<Thing>,
    title: String,
    content: BlogContent,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BlogContent {
    pub time: String,
    pub blocks: Vec<BlogBlock>,
    pub version: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BlogBlock {
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub data: BlogBlockData,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
pub struct BlogBlockData {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<String>>,
}

impl Blog {
    pub fn new(blog_title: String, blog_content: BlogContent) -> Self {
        Self {
            id: None,
            title: blog_title,
            content: blog_content,
        }
    }

    pub async fn save(&self, db: Arc<Surreal<Client>>, user: Option<Thing>) -> Result<(), String> {
        match user.clone() {
            None => {
                println!("Error: {:?}", "No user provided");
                return Err(format!("{:?}", "User details cannot be found"));
            }
            Some(_) => {}
        }

        let response: Result<Option<Self>, surrealdb::Error> =
            db.create("blog").content(self).await;

        match response {
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(format!("{:?}", e));
            }
            Ok(_) => {}
        }

        let blog = response.unwrap();

        match blog {
            None => {
                println!("Error: {:?}", "No blog returned");
                return Err(format!("{:?}", "No blog returned"));
            }
            Some(blog) => {
                Self::relate_user_with_blog(db.clone(), blog.id.unwrap(), user.unwrap()).await?;
            }
        }

        Ok(())
    }

    async fn relate_user_with_blog(
        db: Arc<Surreal<Client>>,
        blog_id: Thing,
        user_id: Thing,
    ) -> Result<(), String> {
        let query =
            get_relate_query_with_content(user_id, blog_id, "create_blog".to_string(), None);

        let response = db.query(query).await;

        match response {
            Err(e) => {
                println!("Error: {:?}", e);
                Err(format!("{:?}", e))
            }
            Ok(_) => Ok(()),
        }
    }
}
