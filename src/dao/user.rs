use chrono::Utc;
use nightmare_common::log;
use nightmare_common::models::{users, permissions, permission_user, role_user, roles};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, QueryTrait, Set, Condition};
use sea_orm::prelude::*;

pub async fn email_exist_except<T: AsRef<str>>(
    db: &DatabaseConnection,
    id: &Uuid,
    email: T,
) -> bool {
    users::Entity::find()
        .filter(users::Column::Email.eq(email.as_ref()))
        .filter(users::Column::Id.ne(id.clone()))
        .count(db)
        .await
        .unwrap()
        .gt(&0u64)
}

pub async fn username_exist_except<T: AsRef<str>>(
    db: &DatabaseConnection,
    id: &Uuid,
    username: T,
) -> bool {
    users::Entity::find()
        .filter(users::Column::Username.eq(username.as_ref()))
        .filter(users::Column::Id.ne(id.clone()))
        .count(db)
        .await
        .unwrap()
        .gt(&0u64)
}

pub async fn email_exist<T: AsRef<str>>(
    db: &DatabaseConnection,
    email: T,
) -> bool {
    users::Entity::find()
        .filter(users::Column::Email.eq(email.as_ref()))
        .count(db)
        .await
        .unwrap()
        .gt(&0u64)
}

pub async fn username_exist<T: AsRef<str>>(
    db: &DatabaseConnection,
    username: T,
) -> bool {
    users::Entity::find()
        .filter(users::Column::Username.eq(username.as_ref()))
        .count(db)
        .await
        .unwrap()
        .gt(&0u64)
}

pub async fn find(
    db: &DatabaseConnection,
    id: Uuid,
) -> Option<users::Model> {
    users::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap()
}

pub async fn find_by_email_or_username<T: ToString>(
    db: &DatabaseConnection,
    email_or_username: T,
) -> Option<users::Model> {
    let query = users::Entity::find()
        .filter(
            Condition::any()
                .add(users::Column::Email.eq(email_or_username.to_string()))
                .add(users::Column::Username.eq(email_or_username.to_string()))
        );

    log::debug!(find_by_email_or_username, "{}", query.build(db.get_database_backend()));

    let user = query.one(db).await;

    if let Err(e) = user {
        log::error!(find_by_email_or_username, "{}", e);

        return None
    }

    user.unwrap()
}

pub async fn store(
    db: &DatabaseConnection,
    user: users::Model,
) -> Result<users::Model, DbErr> {
    let model = users::ActiveModel::from(user);
    let query = users::Entity::insert(model.clone());

    log::debug!(store, "{}", query.build(db.get_database_backend()));

    model.insert(db).await
}

pub async fn update(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<users::Model, DbErr> {
    let mut model = users::ActiveModel::from(user.clone());

    model.name = Set(user.name.clone());
    model.email = Set(user.email.clone());
    model.username = Set(user.username.clone());
    model.password = Set(user.password.clone());
    model.email_verified_at = Set(user.email_verified_at.clone());
    model.profile_photo_id = Set(user.profile_photo_id.clone());
    model.deleted_at = Set(user.deleted_at.clone());
    model.updated_at = Set(Utc::now().naive_local());

    let query = users::Entity::update(model);

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await
}

pub async fn delete(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<users::Model, DbErr> {
    let mut user = users::ActiveModel::from(user.clone());

    user.deleted_at = Set(Some(Utc::now().naive_local()));

    let query = users::Entity::update(user);

    log::debug!(delete, "{}", query.build(db.get_database_backend()));

    query.exec(db).await
}

pub async fn sync_permissions(
    db: &DatabaseConnection,
    user: &users::Model,
    permissions: Vec<permissions::Model>,
) -> Result<(), DbErr> {
    let exists = permission_user::Entity::find()
        .filter(permission_user::Column::UserId.eq(user.id.clone()))
        .all(db)
        .await
        .unwrap();

    let mut detached = Vec::new();
    let mut attached = Vec::new() ;

    for exist in &exists {
        let mut detach = true;

        for permission in &permissions {
            if exist.permission_id.eq(&permission.id) {
                detach = false;

                break;
            }
        }

        if detach {
            detached.push(exist.clone());
        }
    }

    for permission in &permissions {
        let mut attach = true;

        for exist in &exists {
            if permission.id.eq(&exist.permission_id) {
                attach = false;

                break;
            }
        }

        if attach {
            attached.push(permission.clone());
        }
    }

    if !detached.is_empty() {
        permission_user::Entity::delete_many()
            .filter(permission_user::Column::Id.is_in(
                detached.iter()
                    .map(|detach| detach.id)
                    .collect::<Vec<Uuid>>()
            ))
            .exec(db)
            .await
            .unwrap();
    }

    let query = permission_user::Entity::insert_many(attached.iter().map(|attach| {
        let mut model = permission_user::ActiveModel::new();

        model.id = Set(Uuid::new_v4());
        model.user_id = Set(user.id.clone());
        model.permission_id = Set(attach.id);
        model
    }).collect::<Vec<permission_user::ActiveModel>>());

    log::debug!(sync_permissions, "{}", query.build(db.get_database_backend()).to_string());

    if let Err(e) = query.exec(db).await {
        log::error!(sync_permissions, "{}", e);

        return Err(e)
    }
    
    Ok(())
}


pub async fn sync_roles(
    db: &DatabaseConnection,
    user: &users::Model,
    roles: Vec<roles::Model>,
) -> Result<(), DbErr> {
    let exists = role_user::Entity::find()
        .filter(role_user::Column::UserId.eq(user.id.clone()))
        .all(db)
        .await
        .unwrap();

    let mut detached = Vec::new();
    let mut attached = Vec::new() ;

    for exist in &exists {
        let mut detach = true;

        for role in &roles {
            if exist.role_id.eq(&role.id) {
                detach = false;

                break;
            }
        }

        if detach {
            detached.push(exist.clone());
        }
    }

    for role in &roles {
        let mut attach = true;

        for exist in &exists {
            if role.id.eq(&exist.role_id) {
                attach = false;

                break;
            }
        }

        if attach {
            attached.push(role.clone());
        }
    }

    if !detached.is_empty() {
        role_user::Entity::delete_many()
            .filter(role_user::Column::Id.is_in(
                detached.iter()
                    .map(|detach| detach.id)
                    .collect::<Vec<Uuid>>()
            ))
            .exec(db)
            .await
            .unwrap();
    }

    if attached.is_empty() {
        return Ok(())
    }

    let query = role_user::Entity::insert_many(attached.iter().map(|attach| {
        let mut model = role_user::ActiveModel::new();

        model.id = Set(Uuid::new_v4());
        model.user_id = Set(user.id.clone());
        model.role_id = Set(attach.id);
        model
    }).collect::<Vec<role_user::ActiveModel>>());

    log::debug!(sync_roles, "{}", query.build(db.get_database_backend()).to_string());

    if let Err(e) = query.exec(db).await {
        log::error!(sync_roles, "{}", e);

        return Err(e)
    }
    
    Ok(())
}
