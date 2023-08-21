use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::services::query_builder::get_relate_query_with_content;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Project {
    id: Option<Thing>,
    title: String,
    content: ProjectContent,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProjectContent {
    pub time: String,
    pub blocks: Vec<ProjectBlock>,
    pub version: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProjectBlock {
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub data: ProjectBlockData,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProjectBlockData {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<String>>,
}

impl Project {
    pub fn new(project_title: String, project_content: ProjectContent) -> Self {
        Self {
            id: None,
            title: project_title,
            content: project_content,
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
            db.create("project").content(self).await;

        match response {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(format!("{:?}", e));
            }
        }

        let project = response.unwrap();

        match project {
            None => {
                println!("Error: {:?}", "No blog returned");
                return Err(format!("{:?}", "No blog returned"));
            }
            Some(blog) => {
                Self::relate_user_with_project(db.clone(), blog.id.unwrap(), user.unwrap()).await?;
            }
        }

        Ok(())
    }

    async fn relate_user_with_project(
        db: Arc<Surreal<Client>>,
        project_id: Thing,
        user_id: Thing,
    ) -> Result<(), String> {
        let query =
            get_relate_query_with_content(user_id, project_id, "create_project".to_string(), None);

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
