use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Login {
    #[schema(example = "john")]
    pub email_or_username: String,
    #[schema(example = "Password123")]
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Register {
    #[schema(example = "john")]
    pub name: String,
    #[schema(example = "john@local.app")]
    pub email: String,
    #[schema(example = "john")]
    pub username: String,
    #[schema(example = "Password123")]
    pub password: String,
    #[schema(example = "Password123")]
    pub password_confirmation: String,
}
