
#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct Claim {
    user_id: String,
    user_type: String
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
            user_type: claim.get_user_type()
        }
    }
}