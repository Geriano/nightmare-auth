use nightmare_common::{response::pagination, models::roles};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoResponses};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct RoleOAS {
    #[schema()]
    pub id: Uuid,
    #[schema(example = "SUPERUSER")]
    pub code: String,
    #[schema(example = "superuser")]
    pub name: String,
}

impl From<roles::Model> for RoleOAS {
    fn from(role: roles::Model) -> Self {
        Self {
            id: role.id,
            code: role.code,
            name: role.name,
        }
    }
}

impl From<&roles::Model> for RoleOAS {
    fn from(role: &roles::Model) -> Self {
        Self {
            id: role.id,
            code: role.code.clone(),
            name: role.name.clone(),
        }
    }
}

pagination::create!(RoleOAS);
