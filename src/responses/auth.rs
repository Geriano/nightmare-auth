use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoResponses};

use super::user::UserOAS;

#[derive(Clone, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct Login {
    #[schema()]
    pub token: String,
    #[schema()]
    pub user: UserOAS,
}

#[derive(Clone, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct Authenticated {
    #[schema()]
    pub user: UserOAS,
    #[schema(example = json!(["CREATE_USER", "DELETE_USER"]))]
    pub permissions: Vec<String>,
    #[schema(example = json!(["SUPERUSER", "MANAGER"]))]
    pub roles: Vec<String>,
}
