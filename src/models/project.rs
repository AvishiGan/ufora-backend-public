use std::sync::Arc;

use surrealdb::{ sql::Thing, Surreal, engine::remote::ws::Client };

use crate::services::query_builder::get_relate_query_with_content;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Project {
    id: Option<Thing>,
    title: Option<String>,
    content: Option<ProjectContent>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProjectContent {
    pub time: Option<String>,
    pub blocks: Option<Vec<ProjectBlock>>,
    pub version: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProjectBlock {
    pub id: Option<String>,
    pub block_type: Option<String>,
    pub data: Option<ProjectBlockData>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProjectBlockData {
    text: Option<String>,
    level: Option<i32>,
    style: Option<String>,
    items: Option<Vec<String>>,
}

impl Project {
    pub fn new(project_title: Option<String>, project_content: ProjectContent) -> Self {
        Self {
            id: None,
            title: project_title,
            content: Some(project_content),
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

        match self.title.clone() {
            None => {
                println!("Error: {:?}", "No title provided");
                return Err(format!("{:?}", "No title provided"));
            }
            Some(_) => {}
        }

        let response: Result<Option<Self>, surrealdb::Error> = db
            .create("project")
            .content(self).await;

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
        user_id: Thing
    ) -> Result<(), String> {
        let query = get_relate_query_with_content(
            user_id,
            project_id,
            "create_project".to_string(),
            None
        );

        let response = db.query(query).await;

        match response {
            Err(e) => {
                println!("Error: {:?}", e);
                Err(format!("{:?}", e))
            }
            Ok(_) => { Ok(()) }
        }
    }
}
