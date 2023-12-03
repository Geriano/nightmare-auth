use nightmare_common::log;
use nightmare_common::models::permissions;
use sea_orm::{DatabaseConnection, DbErr, Set, ActiveModelBehavior, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, DeleteResult, QueryTrait, ConnectionTrait};
use uuid::Uuid;

pub async fn find(
    db: &DatabaseConnection,
    id: Uuid,
) -> Option<permissions::Model> {
    permissions::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap()
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
    let permission = permissions::Model {
        id: Uuid::new_v4(),
        code: code.to_string(),
        name: name.to_string(),
    };

    let query = permissions::Entity::insert(permissions::ActiveModel::from(
        permission.clone()
    ));

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await?;

    Ok(permission)
}

pub async fn update<N: ToString>(
    db: &DatabaseConnection,
    id: Uuid,
    name: N,
) -> Result<permissions::Model, DbErr> {
    let mut permission = permissions::ActiveModel::new();

    permission.id = Set(id);
    permission.name = Set(name.to_string());
    
    let query = permissions::Entity::update(permission);

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await
}

pub async fn delete(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<DeleteResult, DbErr> {
    let query = permissions::Entity::delete_by_id(id);

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await
}
