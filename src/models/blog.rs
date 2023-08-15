use std::sync::Arc;

use surrealdb::{ sql::Thing, Surreal, engine::remote::ws::Client };

use crate::services::queryBuilder::{get_relate_query_with_content};

use super::user;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Blog {
    id: Option<Thing>,
    title: Option<String>,
    content: Option<BlogContent>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BlogContent {
    pub time: Option<String>,
    pub blocks: Option<Vec<BlogBlock>>,
    pub version: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BlogBlock {
    pub id: Option<String>,
    pub block_type: Option<String>,
    pub data: Option<BlogBlockData>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BlogBlockData {
    text: Option<String>,
    level: Option<i32>,
    style: Option<String>,
    items: Option<Vec<String>>,
}

impl Blog {
    pub fn new(blog_title: Option<String>,blog_content: BlogContent) -> Self {
        Self {
            id: None,
            title: blog_title,
            content: Some(blog_content),
        }
    }

    pub fn get_blog_content(&self) -> Option<BlogContent> {
        self.content.clone()
    }

    pub async fn save(
        &self, db: Arc<Surreal<Client>>,
        user: Option<Thing>
    ) -> Result<(), String> {

        match user.clone() {
            None => {
                println!("Error: {:?}", "No user provided");
                return Err(format!("{:?}", "User details cannot be found"));
            }
            Some(_) => {}
        }

        match self.title.clone() {
            None => {
                println!("Error: {:?}", "No title provided");
                return Err(format!("{:?}", "No title provided"));
            }
            Some(_) => {}
        }


        let response: Result<Option<Self>, surrealdb::Error> = db
            .create("blog")
            .content(self).await;

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
        user_id: Thing
    ) -> Result<(), String> {

        let query = get_relate_query_with_content(user_id, blog_id, "create_blog".to_string(), None);

        let response = db.query(query).await;

        match response {
            Err(e) => {
                println!("Error: {:?}", e);
                Err(format!("{:?}", e))
            }
            Ok(_) => { Ok(())}
        }

    }
}
