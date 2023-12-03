use chrono::NaiveDateTime;
use nightmare_common::{response::pagination, models::users};
use serde::{Deserialize, Serialize};
use utoipa::{IntoResponses, ToSchema};
use uuid::Uuid;


#[derive(Clone, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 201, description = "Created")]
#[serde(rename_all = "camelCase")]
pub struct Created {
    #[schema()]
    pub id: Uuid,
    #[schema(example = "User has been created")]
    pub message: String,
}

#[derive(Clone, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
#[serde(rename_all = "camelCase")]
pub struct UserOAS {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub name: String,
    #[schema()]
    pub email: String,
    #[schema()]
    pub username: String,
    #[schema()]
    pub email_verified_at: Option<NaiveDateTime>,
    #[schema()]
    pub profile_photo_id: Option<String>,
}

impl From<&users::Model> for UserOAS {
    fn from(user: &users::Model) -> Self {
        Self {
            id: user.id,
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
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
            email_verified_at: user.email_verified_at,
            profile_photo_id: user.profile_photo_id,
        }
    }
}

pagination::create!(UserOAS);
