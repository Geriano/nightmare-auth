use nightmare_common::time;
use nightmare_common::models::{users, permissions, permission_user, role_user, roles};
use sea_orm::{Set, Condition};
use sea_orm::prelude::*;

type Id = nightmare_common::models::Id;

pub async fn email_exist_except<T: AsRef<str>, I: Into<Id> + Clone>(
    db: &DatabaseConnection,
    id: &I,
    email: T,
) -> bool {
    let id: Id = id.clone().into();
    
    users::Entity::find()
        .filter(users::Column::Email.eq(email.as_ref()))
        .filter(users::Column::Id.ne(id))
        .count(db)
        .await
        .unwrap()
        .gt(&0u64)
}

pub async fn username_exist_except<T: AsRef<str>, I: Into<Id> + Clone>(
    db: &DatabaseConnection,
    id: &I,
    username: T,
) -> bool {
    let id: Id = id.clone().into();

    users::Entity::find()
        .filter(users::Column::Username.eq(username.as_ref()))
        .filter(users::Column::Id.ne(id))
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

pub async fn find<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> Option<users::Model> {
    let id = id.into();

    users::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap_or(None)
}

pub async fn find_by_email_or_username<T: ToString>(
    db: &DatabaseConnection,
    email_or_username: T,
) -> Option<users::Model> {
    users::Entity::find()
        .filter(
            Condition::any()
                .add(users::Column::Email.eq(email_or_username.to_string()))
                .add(users::Column::Username.eq(email_or_username.to_string()))
        )
        .one(db)
        .await
        .unwrap_or(None)
}

pub async fn store(
    db: &DatabaseConnection,
    user: users::Model,
) -> Result<users::Model, DbErr> {
    users::ActiveModel::from(user)
        .insert(db)
        .await
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
    model.updated_at = Set(time::now());
    model.update(db).await
}

pub async fn delete(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<users::Model, DbErr> {
    let mut user = users::ActiveModel::from(user.clone());

    user.deleted_at = Set(Some(time::now()));
    user.update(db).await
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
                    .map(|detach| detach.id.clone().into())
                    .collect::<Vec<Id>>()
            ))
            .exec(db)
            .await?;
    }

    if !attached.is_empty() {
        permission_user::Entity::insert_many(
            attached.iter().map(|attach| {
                let mut model = permission_user::ActiveModel::new();
    
                model.id = Set(Uuid::new_v4().into());
                model.user_id = Set(user.id.clone());
                model.permission_id = Set(attach.id.clone());
                model
            }).collect::<Vec<permission_user::ActiveModel>>()
        ).exec(db).await?;
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
                    .map(|detach| detach.id.clone().into())
                    .collect::<Vec<Id>>()
            ))
            .exec(db)
            .await?;
    }

    if !attached.is_empty() {
        role_user::Entity::insert_many(
            attached.iter().map(|attach| {
                let mut model = role_user::ActiveModel::new();
        
                let id = Uuid::new_v4();
                model.user_id = Set(id.into());
                model.role_id = Set(attach.id.clone());
                model
            }).collect::<Vec<role_user::ActiveModel>>()
        ).exec(db).await?;
    }
    
    Ok(())
}
