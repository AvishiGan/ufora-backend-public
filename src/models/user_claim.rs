use async_trait::async_trait;
use axum::{ extract::FromRequestParts, http::request::Parts };
use surrealdb::sql::{ Thing, Id };

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claim {
    user_id: String,
    user_type: String,
}

impl Claim {
    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn get_user_type(&self) -> String {
        self.user_type.clone()
    }

    pub fn from(claim: crate::services::jwt::Claim) -> Self {
        Self {
            user_id: claim.get_user_id(),
            user_type: claim.get_user_type(),
        }
    }

    pub fn to_owned(&self) -> Self {
        Self {
            user_id: self.user_id.clone(),
            user_type: self.user_type.clone(),
        }
    }

    pub fn get_surrealdb_thing(&self) -> Thing {
        Thing {
            tb: "user".to_string(),
            id: Id::String(self.user_id.clone()),
        }
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claim {
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, String> {
        let claim = parts.extensions.get::<Claim>().ok_or("Missing claim")?;

        Ok(claim.to_owned())
    }
}
