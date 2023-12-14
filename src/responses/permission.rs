use nightmare_common::response::pagination;
use nightmare_common::models::{permissions, Id};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoResponses};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct PermissionOAS {
    #[schema(example = json!(Uuid::new_v4().to_string()))]
    pub id: Id,
    #[schema(example = "CREATE_USER")]
    pub code: String,
    #[schema(example = "create user")]
    pub name: String,
}

impl From<permissions::Model> for PermissionOAS {
    fn from(permission: permissions::Model) -> Self {
        Self {
            id: permission.id.clone(),
            code: permission.code,
            name: permission.name,
        }
    }
}

impl From<&permissions::Model> for PermissionOAS {
    fn from(permission: &permissions::Model) -> Self {
        Self {
            id: permission.id.clone(),
            code: permission.code.clone(),
            name: permission.name.clone(),
        }
    }
}

pagination::create!(PermissionOAS);
