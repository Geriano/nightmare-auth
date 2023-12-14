use std::collections::HashMap;
use std::str::FromStr;

use actix_web::{HttpResponse, ResponseError};
use chrono::NaiveDateTime;
use nightmare_common::{base58, hash, log};
use nightmare_common::hash::Hash;
use nightmare_common::middleware::auth::Auth;
use nightmare_common::models::{Id, QUERY_BUILDER, permission_user, permissions, role_user, roles, tokens, users};
use nightmare_common::response::http::Unauthorized;
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityName, FromQueryResult, IntoIdentity, Statement};
use sea_query::{Expr, Iden, IntoIden, Query, SelectStatement};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{requests::auth::Login, dao::{user, self}, responses::user::UserOAS};

pub async fn login(
    db: &DatabaseConnection,
    request: Login,
) -> HttpResponse {
    log::info!(login, "{}", request.email_or_username);
    let mut validation = HashMap::new();
    let email_or_username = request.email_or_username.trim().to_lowercase();
    let password = request.password;
    let mut user = None;

    if email_or_username.is_empty() {
        validation.insert("email_or_username", vec!["field email or username is required"]);
    } else {
        let mut errors = vec![];

        user = user::find_by_email_or_username(db, email_or_username.clone()).await;

        if user.is_none() {
            errors.push("email or username doesn't exist");
        }

        if !errors.is_empty() {
            validation.insert("email_or_username", errors);
        }
    }

    if password.is_empty() {
        validation.insert("password", vec!["password field is required"]);
    } else {
        let mut errors = vec![];

        if let Some(user) = user.clone() {
            let hashed = Hash::from(user.password);

            if !hash::verify(hashed, user.id, password) {
                errors.push("wrong password");
            }
        }

        if !errors.is_empty() {
            validation.insert("password", errors);
        }
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }

    let user = user.unwrap();

    match dao::auth::generate(db, &user, None).await {
        Err(e) => {
            log::error!(services::auth::login, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(token) => {
            let response = json!({
                "token": base58::to_string(token.id.as_bytes()),
                "user": UserOAS::from(user),
            });

            HttpResponse::Ok().json(response)
        },
    }
}

pub async fn authenticate(auth: Auth) -> HttpResponse {
    HttpResponse::Ok().json(auth)
}

pub async fn logout(
    db: &DatabaseConnection,
    auth: Auth,
) -> HttpResponse {
    match dao::auth::delete(db, auth.user.id).await {
        Err(e) => {
            log::error!(services::auth::logout, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
    }
}

pub async fn authenticate_by_token(
    db: &DatabaseConnection,
    token: String,
) -> HttpResponse {
    let token = base58::decode(token);
    
    if let Err(e) = token {
        log::error!(services::auth::authenticate_by_token, "{}", e);

        return Unauthorized {
            message: e.to_string(),
        }.error_response();
    }

    let token = token.unwrap();
    #[cfg(feature = "postgres")]
    let id = Uuid::from_slice(&token);
    #[cfg(feature = "sqlite")]
    let id = Uuid::from_str(&String::from_utf8_lossy(&token));

    if let Err(e) = id {
        log::error!(services::auth::authenticate_by_token, "{}", e);

        return Unauthorized {
            message: e.to_string(),
        }.error_response();
    }

    let query = query(id.unwrap().into());
    let statement = Statement::from_string(db.get_database_backend(),query.to_string(QUERY_BUILDER));
    let rows = Value::find_by_statement(statement).all(db).await;

    if let Err(e) = rows {
        log::error!(authentication, "{}", e.to_string());

        return Unauthorized { 
            message: e.to_string(),
        }.error_response()
    }

    let rows = rows.unwrap();

    if rows.is_empty() {
        return Unauthorized { 
            message: "Invalid token, record not found".to_string(),
        }.error_response()
    }

    let mut user= None;
    let mut permissions = vec![];
    let mut roles = vec![];

    for row in rows {
        user = Some(users::Model {
            id: serde_json::from_value(row["user_id"].clone()).unwrap(),
            name: serde_json::from_value(row["name"].clone()).unwrap(),
            email: serde_json::from_value(row["email"].clone()).unwrap(),
            email_verified_at: serde_json::from_value::<NaiveDateTime>(row["email_verified_at"].clone()).map(|date| Some(date.and_utc())).unwrap_or(None),
            username: serde_json::from_value(row["username"].clone()).unwrap(),
            password: serde_json::from_value(row["password"].clone()).unwrap(),
            profile_photo_id: serde_json::from_value(row["profile_photo_id"].clone()).unwrap(),
            created_at: serde_json::from_value::<NaiveDateTime>(row["created_at"].clone()).unwrap().and_utc(),
            updated_at: serde_json::from_value::<NaiveDateTime>(row["updated_at"].clone()).unwrap().and_utc(),
            deleted_at: serde_json::from_value(row["deleted_at"].clone()).unwrap_or(None),
        });

        if !row["permission_id"].is_null() {
            permissions.push(permissions::Model {
                id: serde_json::from_value(row["permission_id"].clone()).unwrap(),
                code: serde_json::from_value(row["permission_code"].clone()).unwrap(),
                name: serde_json::from_value(row["permission_name"].clone()).unwrap(),
            });
        }

        if !row["role_id"].is_null() {
            roles.push(roles::Model {
                id: serde_json::from_value(row["role_id"].clone()).unwrap(),
                code: serde_json::from_value(row["role_code"].clone()).unwrap(),
                name: serde_json::from_value(row["role_name"].clone()).unwrap(),
            });
        }
    }

    let auth = Auth { user: user.unwrap(), permissions, roles };

    HttpResponse::Ok().json(auth)
}

fn query(id: Id) -> SelectStatement {
    Query::select()
        .exprs([
            Expr::col((tokens::Entity.table_name().into_identity(), tokens::Column::UserId.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::Id.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::Name.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::Email.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::EmailVerifiedAt.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::Username.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::Password.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::ProfilePhotoId.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::CreatedAt.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::UpdatedAt.into_iden())),
            Expr::col((users::Entity.table_name().into_identity(), users::Column::DeletedAt.into_iden())),
            Expr::col((
                permission_user::Entity.table_name().into_identity(), 
                permission_user::Column::PermissionId.into_iden()
            )),
            Expr::custom_keyword(format!(
                "{}.{} as {}", 
                permissions::Entity.table_name(), 
                permissions::Column::Code.to_string(), 
                "permission_code"
            ).into_identity()),
            Expr::custom_keyword(format!(
                "{}.{} as {}", 
                permissions::Entity.table_name(), 
                permissions::Column::Name.to_string(), 
                "permission_name"
            ).into_identity()),
            Expr::col((role_user::Entity.table_name().into_identity(), role_user::Column::RoleId.into_iden())),
            Expr::custom_keyword(format!(
                "{}.{} as {}", 
                roles::Entity.table_name(), 
                roles::Column::Code.to_string(), 
                "role_code"
            ).into_identity()),
            Expr::custom_keyword(format!(
                "{}.{} as {}", 
                roles::Entity.table_name(), 
                roles::Column::Name.to_string(), 
                "role_name"
            ).into_identity()),
        ])
        .from(tokens::Entity.table_name().into_identity())
        .inner_join(
            users::Entity.table_name().into_identity(), 
            Expr::col((
                users::Entity.table_name().into_identity(), 
                users::Column::Id.into_iden(),
            )).eq(Expr::col((
                tokens::Entity.table_name().into_identity(), 
                tokens::Column::UserId.into_iden(),
            )))
        )
        .left_join(
            permission_user::Entity.table_name().into_identity(), 
            Expr::col((
                users::Entity.table_name().into_identity(),
                users::Column::Id.into_iden(),
            )).eq(Expr::col((
                permission_user::Entity.table_name().into_identity(),
                permission_user::Column::UserId.into_iden(),
            )))
        )
        .left_join(
            permissions::Entity.table_name().into_identity(), 
            Expr::col((
                permissions::Entity.table_name().into_identity(),
                permissions::Column::Id.into_iden(),
            )).eq(Expr::col((
                permission_user::Entity.table_name().into_identity(),
                permission_user::Column::PermissionId.into_iden(),
            )))
        )
        .left_join(
            role_user::Entity.table_name().into_identity(), 
            Expr::col((
                users::Entity.table_name().into_identity(),
                users::Column::Id.into_iden(),
            )).eq(Expr::col((
                role_user::Entity.table_name().into_identity(),
                role_user::Column::UserId.into_iden(),
            )))
        )
        .left_join(
            roles::Entity.table_name().into_identity(), 
            Expr::col((
                roles::Entity.table_name().into_identity(),
                roles::Column::Id.into_iden(),
            )).eq(Expr::col((
                role_user::Entity.table_name().into_identity(),
                role_user::Column::RoleId.into_iden(),
            )))
        )
        .and_where(
            Expr::col((
                tokens::Entity.table_name().into_identity(),
                tokens::Column::Id.into_iden(),
            )).eq(id)
        )
        .take()

}
