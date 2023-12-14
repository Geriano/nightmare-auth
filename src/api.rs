use actix_web::Scope;
use actix_web::web;
use actix_web::web::redirect;
use nightmare_common::api::{Authentication, Common};
use nightmare_common::request::pagination::Request as PaginationRequest;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::controllers;
use crate::requests;
use crate::requests::user::UserOrderByColumn;
use crate::requests::permission::PermissionOrderByColumn;
use crate::requests::role::RoleOrderByColumn;
use crate::responses;

#[derive(OpenApi)]
#[openapi(
    modifiers(&Authentication, &Common),
    info(
        title = "Auth",
        description = "Authentication Service",
        contact(
            name = "Geriano",
            email = "gerznewbie@gmail.com",
            url = "geriano.github.io",
        ),
    ),
    tags(
        (name = "Authentication"),
        (name = "Master User"),
        (name = "Permission"),
        (name = "Role"),
    ),
    paths(
        controllers::auth::login,
        controllers::auth::authenticate,
        controllers::auth::logout,

        controllers::user::paginate,
        controllers::user::store,
        controllers::user::show,
        controllers::user::update_general_information,
        controllers::user::update_password,
        controllers::user::delete,
        controllers::user::sync_permissions,
        controllers::user::sync_roles,

        controllers::permission::paginate,
        controllers::permission::store,
        controllers::permission::show,
        controllers::permission::update,
        controllers::permission::delete,

        controllers::role::paginate,
        controllers::role::store,
        controllers::role::show,
        controllers::role::update,
        controllers::role::delete,
    ),
    components(
        schemas(requests::auth::Login),
        schemas(requests::auth::Register),

        schemas(requests::user::UserOrderByColumn),
        schemas(requests::user::UserStoreRequest),
        schemas(requests::user::UserUpdateGeneralInformationRequest),
        schemas(requests::user::UserUpdatePasswordRequest),

        schemas(requests::permission::PermissionOrderByColumn),
        schemas(requests::permission::PermissionStoreRequest),
        schemas(requests::permission::PermissionUpdateRequest),
        schemas(requests::permission::PermissionBulkRequest),

        schemas(requests::role::RoleOrderByColumn),
        schemas(requests::role::RoleStoreRequest),
        schemas(requests::role::RoleUpdateRequest),
        schemas(requests::role::RoleBulkRequest),

        schemas(responses::user::UserOAS),
        schemas(responses::permission::PermissionOAS),
        schemas(responses::role::RoleOAS),

        schemas(PaginationRequest<UserOrderByColumn>),
        schemas(PaginationRequest<PermissionOrderByColumn>),
        schemas(PaginationRequest<RoleOrderByColumn>),
    ),
)]
pub struct Doc;

pub fn route() -> SwaggerUi {
    SwaggerUi::new("/{_:.*}")
        .urls(vec![
            (Url::new("Auth", "/doc/api.json"), Doc::openapi()),
        ])
}

pub async fn json() -> Result<String, serde_json::error::Error> {
    Doc::openapi().to_json()
}

pub fn service() -> Scope {
    web::scope("/doc")
        .route("/api.json", web::to(json))
        .service(redirect("", "/doc/index.html"))
        .service(route())
}
