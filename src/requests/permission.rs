use nightmare_common::models::Id;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub enum PermissionOrderByColumn {
    Code,
    Name,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct PermissionStoreRequest {
    #[schema(example = "CREATE_USER")]
    pub code: String,
    #[schema(example = "create user")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct PermissionUpdateRequest {
    #[schema(example = "create user")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct PermissionBulkRequest {
    #[schema(example = json!(Uuid::new_v4().to_string()))]
    pub permissions: Vec<Id>,
}
