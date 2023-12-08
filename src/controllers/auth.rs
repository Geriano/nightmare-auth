use actix_web::{web::{Data, Json}, Responder};
use nightmare_common::response::http::{InternalServerError, Ok, Unauthorized};
use nightmare_common::middleware::auth::Auth;
use sea_orm::DatabaseConnection;

use crate::{services, requests::auth::Login, responses};

/// Login by email or username
#[utoipa::path(
    tag = "Authentication",
    responses(
        responses::auth::Login,
        InternalServerError,
    ),
)]
#[post("/login")]
pub async fn login(
    db: Data<DatabaseConnection>,
    request: Json<Login>,
) -> impl Responder {
    services::auth::login(&db, request.into_inner()).await
}

/// Get authenticated user, permissions and roles
#[utoipa::path(
    tag = "Authentication",
    security(("token" = [])),
    responses(
        responses::auth::Authenticated,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/user")]
pub async fn authenticate(
    auth: Auth,
) -> impl Responder {
    services::auth::authenticate(auth).await
}

/// Logout by request authorization token
#[utoipa::path(
    tag = "Authentication",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        InternalServerError,
    ),
)]
#[delete("/logout")]
pub async fn logout(
    db: Data<DatabaseConnection>,
    auth: Auth,
) -> impl Responder {
    services::auth::logout(&db, auth).await
}
