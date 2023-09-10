use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use surrealdb::sql::{Id, Thing};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClubClaim {
    pub club_id: String,
    pub position: String,
}

impl ClubClaim {
    pub fn from(claim: crate::services::jwt::ClubClaim) -> Self {
        Self {
            club_id: claim.get_club_id(),
            position: claim.get_position(),
        }
    }
    pub fn get_surrealdb_thing(&self) -> Thing {
        Thing {
            tb: "user".to_string(),
            id: Id::String(self.club_id.clone()),
        }
    }
    pub fn get_position(&self) -> String {
        self.position.clone()
    }
    pub fn get_club_id(&self) -> String {
        self.club_id.clone()
    }

    pub fn to_owned(&self) -> Self {
        Self {
            club_id: self.club_id.clone(),
            position: self.position.clone(),
        }
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ClubClaim {
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, String> {
        let claim = parts.extensions.get::<ClubClaim>().ok_or("Missing claim")?;

        Ok(claim.to_owned())
    }
}
