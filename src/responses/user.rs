use nightmare_common::response::pagination;
use nightmare_common::models::{users, Id, Timestamp};
use serde::{Deserialize, Serialize};
use utoipa::{IntoResponses, ToSchema};

#[derive(Clone, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 201, description = "Created")]
#[serde(rename_all = "camelCase")]
pub struct Created {
    #[schema(example = json!(Uuid::new_v4().to_string()))]
    pub id: Id,
    #[schema(example = "User has been created")]
    pub message: String,
}

#[derive(Clone, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
#[serde(rename_all = "camelCase")]
pub struct UserOAS {
    #[schema(example = json!(Uuid::new_v4().to_string()))]
    pub id: Id,
    #[schema()]
    pub name: String,
    #[schema()]
    pub email: String,
    #[schema()]
    pub username: String,
    #[schema()]
    pub email_verified_at: Option<Timestamp>,
    #[schema()]
    pub profile_photo_id: Option<String>,
}

impl From<&users::Model> for UserOAS {
    fn from(user: &users::Model) -> Self {
        Self {
            id: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            email_verified_at: user.email_verified_at,
            profile_photo_id: user.profile_photo_id.clone(),
        }
    }
}

impl From<users::Model> for UserOAS {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id.clone(),
            name: user.name,
            email: user.email,
            username: user.username,
            email_verified_at: user.email_verified_at,
            profile_photo_id: user.profile_photo_id,
        }
    }
}

pagination::create!(UserOAS);
