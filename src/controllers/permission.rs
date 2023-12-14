use actix_web::Responder;
use actix_web::web::{Data, Json, Path};
use nightmare_common::middleware::auth::Auth;
use nightmare_common::models::Id;
use nightmare_common::request::pagination::{PaginationRequest, PaginationRequestParam};
use nightmare_common::response::http::{Unauthorized, InternalServerError, NotFound, CreatedWithId, OkWithId, Ok};
use sea_orm::DatabaseConnection;

use crate::requests::permission::{PermissionStoreRequest, PermissionUpdateRequest, PermissionOrderByColumn};
use crate::responses::permission::{PermissionOAS, Pagination};
use crate::services;

/// Permission pagination
#[utoipa::path(
    tag = "Permission",
    context_path = "/api/v1",
    security(("token" = [])),
    params(
        PaginationRequestParam<PermissionOrderByColumn>,
    ),
    responses(
        Pagination,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/permission")]
pub async fn paginate(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: PaginationRequest<PermissionOrderByColumn>,
) -> impl Responder {
    services::permission::paginate(&db, request).await
}

/// Store new permission
#[utoipa::path(
    tag = "Permission",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        CreatedWithId,
        Unauthorized,
        InternalServerError,
    ),
)]
#[post("/permission")]
pub async fn store(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Json<PermissionStoreRequest>,
) -> impl Responder {
    services::permission::store(&db, request.into_inner()).await
}

/// Get permission by id
#[utoipa::path(
    tag = "Permission",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        PermissionOAS,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[get("/permission/{id}")]
pub async fn show(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
) -> impl Responder {
    services::permission::show(&db, id.into_inner()).await
}

/// Update permission by id
#[utoipa::path(
    tag = "Permission",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        OkWithId,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[put("/permission/{id}")]
pub async fn update(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
    request: Json<PermissionUpdateRequest>,
) -> impl Responder {
    services::permission::update(&db, id.into_inner(), request.into_inner()).await
}

/// Delete permission by id
#[utoipa::path(
    tag = "Permission",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[delete("/permission/{id}")]
pub async fn delete(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
) -> impl Responder {
    services::permission::delete(&db, id.into_inner()).await
}
