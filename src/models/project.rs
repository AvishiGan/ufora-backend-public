use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::services::query_builder::{
    get_relate_query_with_content, get_select_query, Column, Item, Expression, ExpressionConnector, Return, get_delete_query_with_conditions,
};

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

    pub async fn get_projects_by_user_id(
        db: Arc<Surreal<Client>>,
        user_id: Thing,
    ) -> Result<Vec<Self>, String> {
        let query = get_select_query(
            Item::Record {
                tb: user_id.tb,
                id: user_id.id.to_string(),
            },
            Column::Specific(vec!["->create_project->project.* as projects".to_string()]),
            None,
            None,
            None,
            None,
            None,
        );

        let response = db.query(query).await;

        #[derive(serde::Deserialize, serde::Serialize, Debug)]
        struct Projects {
            projects: Vec<Project>,
        }

        match response {
            Ok(mut response) => {
                let projects: Result<Option<Projects>, surrealdb::Error> = response.take(0);

                match projects {
                    Ok(projects) => Ok(projects.unwrap().projects),
                    Err(e) => {
                        println!("Error: {:?}", e);
                        Err(format!("{:?}", e))
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                Err(format!("{:?}", e))
            }
        }
    }

    pub async fn delete_a_project_belongs_to_user(
        db: Arc<Surreal<Client>>,
        project_id: String,
        user_id: Thing,
    ) -> Result<(), String> {
        
        let condition = vec![(
            Expression::EdgeExpression(
                "<-create_project<-(user WHERE id = ".to_string() + &user_id.to_string() + ")",
            ),
            ExpressionConnector::End,
        )];

        let delete_query = get_delete_query_with_conditions(
            "project:".to_string() + &project_id,
            condition,
            Some(Return::Before),
        );

        let response = db.query(delete_query).await;

        match response {
            Err(e) => {
                println!("Error: {:?}", e);
                Err(format!("{:?}", e))
            }
            Ok(mut response) =>  {
                let project: Result<Vec<Self>, surrealdb::Error> = response.take(0);
                match project {
                    Ok(project) => {
                        if project.len() == 0 {
                            return Err("Project with given id not found".to_string());
                        }
                        Ok(())
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                        Err(format!("{:?}", e))
                    }
                }
            },
        }
    }
}
