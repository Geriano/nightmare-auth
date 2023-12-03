use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub enum RoleOrderByColumn {
    Code,
    Name,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct RoleStoreRequest {
    #[schema(example = "AREA_MANAGER")]
    pub code: String,
    #[schema(example = "area manager")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct RoleUpdateRequest {
    #[schema(example = "area manager")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct RoleBulkRequest {
    #[schema()]
    pub roles: Vec<Uuid>,
}
