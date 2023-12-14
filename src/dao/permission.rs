use nightmare_common::models::Id;
use nightmare_common::models::permissions;
use sea_orm::Set;
use sea_orm::prelude::*;

pub async fn find<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> Option<permissions::Model> {
    let id: Id = id.into();
    permissions::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap_or(None)
}

pub async fn exist<C: ToString>(
    db: &DatabaseConnection,
    code: C,
) -> bool {
    permissions::Entity::find()
        .filter(permissions::Column::Code.eq(code.to_string()))
        .count(db)
        .await
        .unwrap()
        > 0
}

pub async fn store<C: ToString, N: ToString>(
    db: &DatabaseConnection,
    code: C,
    name: N,
) -> Result<permissions::Model, DbErr> {
    let permission = permissions::ActiveModel::from(permissions::Model {
        id: Uuid::new_v4().into(),
        code: code.to_string(),
        name: name.to_string(),
    });
    
    permission.insert(db).await
}

pub async fn update<N: ToString, I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    name: N,
) -> Result<permissions::Model, DbErr> {
    let mut permission = permissions::ActiveModel::new();

    permission.id = Set(id.into());
    permission.name = Set(name.to_string());
    permission.update(db).await
}

pub async fn delete<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> Result<(), DbErr> {
    let id: Id = id.into();
    
    permissions::Entity::delete_by_id(id)
        .exec(db)
        .await?;

    Ok(())
}
