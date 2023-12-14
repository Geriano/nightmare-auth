use std::collections::HashMap;

use actix_web::HttpResponse;
use nightmare_common::hash::Hash;
use nightmare_common::{log, hash, time};
use nightmare_common::models::{users, permissions, roles, Id};
use nightmare_common::request::pagination::PaginationRequest;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QueryFilter, Condition, ColumnTrait, QuerySelect, PaginatorTrait, ConnectionTrait, QueryTrait};
use serde_json::json;
use uuid::Uuid;

use crate::dao;
use crate::requests::permission::PermissionBulkRequest;
use crate::requests::role::RoleBulkRequest;
use crate::requests::user::{UserUpdateGeneralInformationRequest, UserUpdatePasswordRequest, UserOrderByColumn, UserStoreRequest};
use crate::responses::user::UserOAS;

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<UserOrderByColumn>,
) -> HttpResponse {
    let mut query = users::Entity::find()
        .filter(users::Column::DeletedAt.is_null());

    if request.search.is_some() {
        query = query.filter(
            Condition::any()
                .add(users::Column::Name.like(request.search()))
                .add(users::Column::Username.like(request.search()))
                .add(users::Column::Email.like(request.search()))
        )
    }

    let count = query.clone().count(db).await.unwrap();
    let query = query.limit(Some(request.limit() as u64))
        .offset(Some(request.limit() as u64 * (request.page() as u64 - 1)))
        .order_by(match request.order(UserOrderByColumn::Name) {
            UserOrderByColumn::Name => users::Column::Name,
            UserOrderByColumn::Username => users::Column::Username,
            UserOrderByColumn::Email => users::Column::Email,
        }, request.sort());

    log::debug!(paginate, "{}", query.build(db.get_database_backend()).to_string());
    
    match query.all(db).await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(data) => {
            HttpResponse::Ok().json(json!({
                "total": {
                    "data": count,
                    "page": count / request.page(),
                },
                "data": data.iter()
                  .map(|user| user.into())
                  .collect::<Vec<UserOAS>>(),
            }))
        },
    }
}

pub async fn store(
    db: &DatabaseConnection,
    request: UserStoreRequest,
) -> HttpResponse {
    let mut validation = HashMap::new();

    let name = request.name.trim().to_lowercase();
    let email = request.email.trim().to_lowercase();
    let username = request.username.trim().to_lowercase();
    let password = request.password.trim();

    if name.is_empty() {
        validation.insert("name", vec!["field name is required"]);
    }

    if email.is_empty() {
        validation.insert("email", vec!["field email is required"]);
    } else {
        if dao::user::email_exist(db, &email).await {
            validation.insert("email", vec!["email already used"]);
        }
    }

    if username.is_empty() {
        validation.insert("username", vec!["field username is required"]);
    } else {
        if dao::user::username_exist(db, &username).await {
            validation.insert("username", vec!["username already used"]);
        }
    }

    if password.is_empty() {
        validation.insert("password", vec!["field password is required"]);
    } else {
        let mut errors = vec![];

        if password.len() < 6 {
            errors.push("min length for password is 6");
        }

        if !password.chars().any(|c| c.is_numeric()) {
            errors.push("password must have numeric value");
        }

        if !password.chars().any(|c|c.is_alphabetic()) {
            errors.push("password must have alphabetic value");
        }

        if password.to_lowercase().eq(password) || password.to_uppercase().eq(password) {
            errors.push("password must have lower and upper case");
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

    let id = Uuid::new_v4();
    let password = hash::make(id, password);
    let user = dao::user::store(db, users::Model {
        id: id.into(),
        name,
        email,
        username,
        email_verified_at: None,
        password: password.to_string(),
        profile_photo_id: None,
        created_at: time::now(),
        updated_at: time::now(),
        deleted_at: None,
    });

    match user.await {
        Err(e) => {
            log::error!(store, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(user) => {
            log::debug!(store, "created {}", user.id);

            HttpResponse::Created().json(json!({
                "id": user.id,
                "message": "User has been created",
            }))
        },
    }
}

pub async fn show<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> HttpResponse {
    match dao::user::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(user) => HttpResponse::Ok().json(UserOAS::from(&user)),
    }
}

pub async fn update_general_information<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: UserUpdateGeneralInformationRequest,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish()
    }

    let mut user = user.unwrap();
    let name = request.name.trim().to_lowercase();
    let email = request.email.trim().to_lowercase();
    let username = request.username.trim().to_lowercase();
    let profile_photo_id = request.profile_photo_id;
    let mut validation = HashMap::new();

    if name.is_empty() {
        validation.insert("name", vec!["field name is required"]);
    }

    if email.is_empty() {
        validation.insert("email", vec!["field email is required"]);
    } else {
        if dao::user::email_exist_except(db, &user.id, &email).await {
            validation.insert("email", vec!["email already used"]);
        }
    }

    if username.is_empty() {
        validation.insert("username", vec!["field username is required"]);
    } else {
        if dao::user::username_exist_except(db, &user.id, &username).await {
            validation.insert("username", vec!["username already used"]);
        }
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }

    user.name = name;
    user.email = email;
    user.username = username;
    user.profile_photo_id = profile_photo_id;

    match dao::user::update(db, &user).await {
        Err(e) => {
            log::error!(update_general_information, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(_) => {
            log::debug!(update_general_information, "user.updated {}", user.id);

            HttpResponse::Ok().json(json!({
                "message": "User has been updated",
            }))
        },
    }
}

pub async fn update_password<I: Into<Id> + Clone>(
    db: &DatabaseConnection,
    id: I,
    request: UserUpdatePasswordRequest,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish()
    }
    
    let mut user = user.unwrap();
    let current = request.current_password;
    let new = request.new_password;
    let confirmation = request.password_confirmation;
    let mut validation = HashMap::new();

    if current.is_empty() {
        validation.insert("current", vec!["field current password is required"]);
    } else {
        let mut errors = vec![];

        if current.len() < 6 {
            errors.push("min length for current password is 6");
        }

        if !current.chars().any(|c| c.is_numeric()) {
            errors.push("current must have numeric value");
        }

        if !current.chars().any(|c|c.is_alphabetic()) {
            errors.push("current password must have alphabetic value");
        }

        if current.to_lowercase().eq(&current) || current.to_uppercase().eq(&current) {
            errors.push("current password must have lower and upper case");
        }

        if !errors.is_empty() {
            validation.insert("currentPassword", errors);
        }
    }

    if new.is_empty() {
        validation.insert("new", vec!["field new password is required"]);
    } else {
        let mut errors = vec![];

        if new.len() < 6 {
            errors.push("min length for new password is 6");
        }

        if !new.chars().any(|c| c.is_numeric()) {
            errors.push("new must have numeric value");
        }

        if !new.chars().any(|c|c.is_alphabetic()) {
            errors.push("new password must have alphabetic value");
        }

        if new.to_lowercase().eq(&new) || new.to_uppercase().eq(&new) {
            errors.push("new password must have lower and upper case");
        }

        if current.eq(&new) {
            errors.push("can't update new password with old password");
        }

        if !errors.is_empty() {
            validation.insert("newPassword", errors);
        }
    }
    
    if confirmation.is_empty() {
        validation.insert("confirmation", vec!["field password confirmation is required"]);
    } else {
        let mut errors = vec![];

        if confirmation.len() < 6 {
            errors.push("min length for password confirmation is 6");
        }

        if !confirmation.chars().any(|c| c.is_numeric()) {
            errors.push("confirmation must have numeric value");
        }

        if !confirmation.chars().any(|c|c.is_alphabetic()) {
            errors.push("password confirmation must have alphabetic value");
        }

        if confirmation.to_lowercase().eq(&confirmation) || confirmation.to_uppercase().eq(&confirmation) {
            errors.push("password confirmation must have lower and upper case");
        }

        if confirmation.ne(&new) {
            errors.push("password confirmation does't match with new password");
        }

        if !errors.is_empty() {
            validation.insert("passwordConfirmation", errors);
        }
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }

    let hash = Hash::from(user.password);

    if !hash.verify(hash::make(user.id.clone(), current)) {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": {
                "currentPassword": [
                    "wrong password"
                ],
            }
        }))
    }

    user.password = hash::make(user.id.clone(), new).to_string();

    match dao::user::update(db, &user).await {
        Err(e) => {
            log::error!(update_password, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "user password has been updated",
        })),
    }
}

pub async fn delete<I: Into<Id> + Clone>(
    db: &DatabaseConnection,
    id: I,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish()
    }

    match dao::user::delete(db, &user.unwrap()).await {
        Err(e) => {
            log::error!(delete, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(user) => {
            log::debug!(delete, "user.deleted {}", user.id);

            HttpResponse::Ok().json(json!({
                "message": "user has been deleted",
            }))
        },
    }
}

pub async fn sync_permissions<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: PermissionBulkRequest,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish()
    }

    let user = user.unwrap();
    let permissions = permissions::Entity::find()
        .filter(permissions::Column::Id.is_in(request.permissions));

    log::debug!(sync_permissions, "{}", permissions.build(db.get_database_backend()).to_string());

    let permissions = permissions.all(db).await;
    
    if let Err(e) = permissions {
        log::error!(sync_permissions, "{}", e);

        return HttpResponse::InternalServerError().json(json!({
            "message": e.to_string(),
        }))
    }

    match dao::user::sync_permissions(db, &user, permissions.unwrap()).await {
        Err(e) => {
            log::error!(sync_permissions, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        _ => HttpResponse::Ok().finish(),
    }
}

pub async fn sync_roles<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: RoleBulkRequest,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish()
    }

    let user = user.unwrap();
    let roles = roles::Entity::find()
        .filter(roles::Column::Id.is_in(request.roles));

    log::debug!(sync_roles, "{}", roles.build(db.get_database_backend()).to_string());

    let roles = roles.all(db).await;
    
    if let Err(e) = roles {
        log::error!(sync_roles, "{}", e);

        return HttpResponse::InternalServerError().json(json!({
            "message": e.to_string(),
        }))
    }

    match dao::user::sync_roles(db, &user, roles.unwrap()).await {
        Err(e) => {
            log::error!(sync_roles, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        _ => HttpResponse::Ok().finish(),
    }
}
