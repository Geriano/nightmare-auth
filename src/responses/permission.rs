use nightmare_common::{response::pagination, models::permissions};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoResponses};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct PermissionOAS {
    #[schema()]
    pub id: Uuid,
    #[schema(example = "CREATE_USER")]
    pub code: String,
    #[schema(example = "create user")]
    pub name: String,
}

impl From<permissions::Model> for PermissionOAS {
    fn from(permission: permissions::Model) -> Self {
        Self {
            id: permission.id,
            code: permission.code,
            name: permission.name,
        }
    }
}

impl From<&permissions::Model> for PermissionOAS {
    fn from(permission: &permissions::Model) -> Self {
        Self {
            id: permission.id,
            code: permission.code.clone(),
            name: permission.name.clone(),
        }
    }
}

pagination::create!(PermissionOAS);
