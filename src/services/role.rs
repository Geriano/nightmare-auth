use std::collections::HashMap;

use actix_web::HttpResponse;
use nightmare_common::models::{roles, Id};
use nightmare_common::log;
use nightmare_common::request::pagination::PaginationRequest;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QueryFilter, Condition, ColumnTrait, PaginatorTrait, QuerySelect, QueryTrait, ConnectionTrait};
use serde_json::json;

use crate::dao;
use crate::requests::role::{RoleOrderByColumn, RoleStoreRequest, RoleUpdateRequest};
use crate::responses::role::RoleOAS;

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<RoleOrderByColumn>,
) -> HttpResponse {
    let mut query = roles::Entity::find()
        .order_by(match request.order(RoleOrderByColumn::Name) {
            RoleOrderByColumn::Code => roles::Column::Code,
            RoleOrderByColumn::Name => roles::Column::Name,
        }, request.sort());

    if request.search.is_some() {
        query = query.filter(
            Condition::any()
                .add(roles::Column::Code.like(request.search()))
                .add(roles::Column::Name.like(request.search()))
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
        Ok(data) => {
            HttpResponse::Ok().json(json!({
                "total": {
                    "data": count,
                    "page": count / request.page(),
                },
                "data": data.iter()
                   .map(|role| role.into())
                   .collect::<Vec<RoleOAS>>(),
            }))
        },
    }
}

pub async fn store(
    db: &DatabaseConnection,
    request: RoleStoreRequest,
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

    if dao::role::exist(db, &code).await {
        validation.insert("code", vec!["code already exists"]);
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }

    match dao::role::store(db, code, name).await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(role) => {
            HttpResponse::Created().json(json!({
                "id": role.id,
                "message": "Role has been created",
            }))
        }
    }
}

pub async fn show<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> HttpResponse {
    match dao::role::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(role) => HttpResponse::Ok().json(RoleOAS::from(role)),
    }
}

pub async fn update<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: RoleUpdateRequest,
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

    match dao::role::update(db, id, name).await {
        Err(e) => {
            log::error!(update, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(role) => {
            HttpResponse::Ok().json(json!({
                "id": role.id,
                "message": "Role has been updated",
            }))
        }
    }
}

pub async fn delete<I: Into<Id> + Clone>(
    db: &DatabaseConnection,
    id: I,
) -> HttpResponse {
    match dao::role::find(db, id.clone()).await {
        None => HttpResponse::NotFound().finish(),
        Some(role) => match dao::role::delete(db, id).await {
            Err(e) => {
                log::error!(delete, "{}", e);

                HttpResponse::InternalServerError().json(json!({
                    "message": e.to_string(),
                }))
            },
            _ => {
                HttpResponse::Ok().json(json!({
                    "id": role.id,
                    "message": "Role has been deleted",
                }))
            },
        }
    }
}
