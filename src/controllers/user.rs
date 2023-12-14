use actix_web::Responder;
use actix_web::web::{Data, Json, Path};
use nightmare_common::middleware::auth::Auth;
use nightmare_common::models::Id;
use nightmare_common::request::pagination::{PaginationRequest, PaginationRequestParam};
use nightmare_common::response::http::{Ok, InternalServerError, Unauthorized, UnprocessableEntity, NotFound};
use sea_orm::DatabaseConnection;

use crate::requests::permission::PermissionBulkRequest;
use crate::requests::role::RoleBulkRequest;
use crate::requests::user::{UserOrderByColumn, UserStoreRequest, UserUpdateGeneralInformationRequest, UserUpdatePasswordRequest};
use crate::responses::user::{Pagination, UserOAS, Created};
use crate::services;

/// user pagination
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    params(
        PaginationRequestParam<UserOrderByColumn>,
    ),
    responses(
        Pagination,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/user")]
pub async fn paginate(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: PaginationRequest<UserOrderByColumn>,
) -> impl Responder {
    services::user::paginate(&db, request).await
}

/// store new user
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Created,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[post("/user")]
pub async fn store(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Json<UserStoreRequest>,
) -> impl Responder {
    services::user::store(&db, request.into_inner()).await
}

/// show user by id
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        UserOAS,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[get("/user/{id}")]
pub async fn show(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
) -> impl Responder {
    services::user::show(&db, id.to_owned(),).await
}

/// update user by id
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[put("/user/{id}")]
pub async fn update_general_information(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
    request: Json<UserUpdateGeneralInformationRequest>,
) -> impl Responder {
    services::user::update_general_information(&db, id.to_owned(), request.into_inner()).await
}

/// update user password by id
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Ok,
        Unauthorized,
        NotFound,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[patch("/user/{id}")]
pub async fn update_password(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
    request: Json<UserUpdatePasswordRequest>,
) -> impl Responder {
    services::user::update_password(&db, id.to_owned(), request.into_inner()).await
}

/// delete user by id
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[delete("/user/{id}")]
pub async fn delete(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
) -> impl Responder {
    services::user::delete(&db, id.to_owned()).await
}

/// sync user permissions
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[put("/user/{id}/permissions")]
pub async fn sync_permissions(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
    request: Json<PermissionBulkRequest>,
) -> impl Responder {
    services::user::sync_permissions(&db, id.into_inner(), request.into_inner()).await
}

/// sync user roles
#[utoipa::path(
    tag = "Master User",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[put("/user/{id}/roles")]
pub async fn sync_roles(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Id>,
    request: Json<RoleBulkRequest>,
) -> impl Responder {
    services::user::sync_roles(&db, id.into_inner(), request.into_inner()).await
}
