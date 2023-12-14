use nightmare_common::response::pagination;
use nightmare_common::models::{roles, Id};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoResponses};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct RoleOAS {
    #[schema(example = json!(Uuid::new_v4().to_string()))]
    pub id: Id,
    #[schema(example = "SUPERUSER")]
    pub code: String,
    #[schema(example = "superuser")]
    pub name: String,
}

impl From<roles::Model> for RoleOAS {
    fn from(role: roles::Model) -> Self {
        Self {
            id: role.id.clone(),
            code: role.code,
            name: role.name,
        }
    }
}

impl From<&roles::Model> for RoleOAS {
    fn from(role: &roles::Model) -> Self {
        Self {
            id: role.id.clone(),
            code: role.code.clone(),
            name: role.name.clone(),
        }
    }
}

pagination::create!(RoleOAS);
