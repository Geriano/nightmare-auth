use nightmare_common::models::Id;
use nightmare_common::models::roles;
use sea_orm::Set;
use sea_orm::prelude::*;

pub async fn find<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> Option<roles::Model> {
    let id: Id = id.into();

    roles::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap_or(None)
}

pub async fn exist<C: ToString>(
    db: &DatabaseConnection,
    code: C,
) -> bool {
    roles::Entity::find()
        .filter(roles::Column::Code.eq(code.to_string()))
        .count(db)
        .await
        .unwrap()
        > 0
}

pub async fn store<C: ToString, N: ToString>(
    db: &DatabaseConnection,
    code: C,
    name: N,
) -> Result<roles::Model, DbErr> {
    let role = roles::ActiveModel::from(roles::Model {
        id: Uuid::new_v4().into(),
        code: code.to_string(),
        name: name.to_string(),
    });
    
    role.insert(db).await
}

pub async fn update<N: ToString, I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    name: N,
) -> Result<roles::Model, DbErr> {
    let mut role = roles::ActiveModel::new();

    role.id = Set(id.into());
    role.name = Set(name.to_string());
    role.update(db).await
}

pub async fn delete<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> Result<(), DbErr> {
    let id: Id = id.into();
    
    roles::Entity::delete_by_id(id)
        .exec(db)
        .await?;

    Ok(())
}
