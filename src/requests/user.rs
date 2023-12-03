use nightmare_common::models::users;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserOrderByColumn {
    Name,
    Username,
    Email,
}

impl From<UserOrderByColumn> for users::Column {
    fn from(value: UserOrderByColumn) -> Self {
        use UserOrderByColumn::*;

        match value {
            Name => Self::Name,
            Username => Self::Username,
            Email => Self::Email,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct UserStoreRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@local.id")]
    pub email: String,
    #[schema(example = "john")]
    pub username: String,
    #[schema(example = "password")]
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateGeneralInformationRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@local.id")]
    pub email: String,
    #[schema(example = "john")]
    pub username: String,
    #[schema()]
    pub profile_photo_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdatePasswordRequest {
    #[schema(example = "Password123")]
    pub current_password: String,
    #[schema(example = "Password123")]
    pub new_password: String,
    #[schema(example = "Password123")]
    pub password_confirmation: String,
}

