use actix_web::Responder;
use actix_web::web::{Data, Json, Path};
use nightmare_common::middleware::auth::Auth;
use nightmare_common::models::Id;
use nightmare_common::request::pagination::{PaginationRequest, PaginationRequestParam};
use nightmare_common::response::http::{Unauthorized, InternalServerError, NotFound, CreatedWithId, OkWithId, Ok};
use sea_orm::DatabaseConnection;

use crate::requests::role::{RoleStoreRequest, RoleUpdateRequest};
use crate::responses::role::RoleOAS;
use crate::{requests::role::RoleOrderByColumn, responses::role::Pagination};
use crate::services;

/// Role pagination
#[utoipa::path(
    tag = "Role",
    context_path = "/api/v1",
    security(("token" = [])),
    params(
        PaginationRequestParam<RoleOrderByColumn>,
    ),
    responses(
        Pagination,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/role")]
pub async fn paginate(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: PaginationRequest<RoleOrderByColumn>,
) -> impl Responder {
    services::role::paginate(&db, request).await
}

/// Store new role
#[utoipa::path(
    tag = "Role",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        CreatedWithId,
        Unauthorized,
        InternalServerError,
    ),
)]
#[post("/role")]
pub async fn store(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Json<RoleStoreRequest>,
) -> impl Responder {
    services::role::store(&db, request.into_inner()).await
}

/// Get role by id
#[utoipa::path(
    tag = "Role",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        RoleOAS,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[get("/role/{id}")]
pub async fn show(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
) -> impl Responder {
    services::role::show(&db, id.into_inner()).await
}

/// Update role by id
#[utoipa::path(
    tag = "Role",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        OkWithId,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[put("/role/{id}")]
pub async fn update(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
    request: Json<RoleUpdateRequest>,
) -> impl Responder {
    services::role::update(&db, id.into_inner(), request.into_inner()).await
}

/// Delete role by id
#[utoipa::path(
    tag = "Role",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[delete("/role/{id}")]
pub async fn delete(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
) -> impl Responder {
    services::role::delete(&db,  id.into_inner()).await
}
