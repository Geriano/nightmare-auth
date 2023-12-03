use std::collections::HashMap;

use actix_web::HttpResponse;
use nightmare_common::{request::pagination::PaginationRequest, log};
use nightmare_common::models::permissions;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QueryFilter, Condition, ColumnTrait, PaginatorTrait, QuerySelect, QueryTrait, ConnectionTrait};
use serde_json::json;
use uuid::Uuid;

use crate::dao;
use crate::requests::permission::{PermissionOrderByColumn, PermissionStoreRequest, PermissionUpdateRequest};
use crate::responses::permission::PermissionOAS;

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<PermissionOrderByColumn>,
) -> HttpResponse {
    let mut query = permissions::Entity::find()
        .order_by(match request.order(PermissionOrderByColumn::Name) {
            PermissionOrderByColumn::Code => permissions::Column::Code,
            PermissionOrderByColumn::Name => permissions::Column::Name,
        }, request.sort());

    if request.search.is_some() {
        query = query.filter(
            Condition::any()
                .add(permissions::Column::Code.like(request.search()))
                .add(permissions::Column::Name.like(request.search()))
        )
    }

    let count = query.clone().count(db).await.unwrap();
    let query = query.limit(Some(request.limit().into()))
        .offset(Some(request.limit() as u64 * (request.page() as u64 - 1)));

    log::debug!(paginate, "{}", query.build(db.get_database_backend()).to_string());

    match query.all(db).await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(data) => HttpResponse::Ok().json(json!({
            "total": {
                "data": count,
                "page": count / request.page(),
            },
            "data": data.iter()
                .map(|permission| permission.into())
                .collect::<Vec<PermissionOAS>>(),
        })),
    }
}

pub async fn store(
    db: &DatabaseConnection,
    request: PermissionStoreRequest,
) -> HttpResponse {
    let mut validation = HashMap::new();
    let code = request.code.trim().to_uppercase();
    let name = request.name.trim().to_lowercase();

    if code.is_empty() {
        validation.insert("code", vec!["field code is required"]);
    }
    
    if name.is_empty() {
        validation.insert("name", vec!["field name is required"]);
    }

    if dao::permission::exist(db, &code).await {
        validation.insert("code", vec!["code already exists"]);
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }

    match dao::permission::store(db, code, name).await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(permission) => {
            HttpResponse::Created().json(json!({
                "id": permission.id,
                "message": "Permission has been created",
            }))
        }
    }
}

pub async fn show(
    db: &DatabaseConnection,
    id: Uuid,
) -> HttpResponse {
    match dao::permission::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(permission) => HttpResponse::Ok().json(json!(
            PermissionOAS::from(permission)
        )),
    }
}

pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    request: PermissionUpdateRequest,
) -> HttpResponse {
    let mut validation = HashMap::new();
    let name = request.name.trim().to_lowercase();

    if name.is_empty() {
        validation.insert("name", vec!["field name is required"]);
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }

    match dao::permission::update(db, id, name).await {
        Err(e) => {
            log::error!(update, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(permission) => {
            HttpResponse::Ok().json(json!({
                "id": permission.id,
                "message": "Permission has been updated",
            }))
        }
    }
}

pub async fn delete(
    db: &DatabaseConnection,
    id: Uuid,
) -> HttpResponse {
    match dao::permission::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(permission) => match dao::permission::delete(db, id).await {
            Err(e) => {
                log::error!(delete, "{}", e);

                HttpResponse::InternalServerError().json(json!({
                    "message": e.to_string(),
                }))
            },
            _ => {
                HttpResponse::Ok().json(json!({
                    "id": permission.id,
                    "message": "Permission has been deleted",
                }))
            },
        }
    }
}
