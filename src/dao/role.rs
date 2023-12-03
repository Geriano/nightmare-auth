use nightmare_common::log;
use nightmare_common::models::roles;
use sea_orm::{DatabaseConnection, DbErr, Set, ActiveModelBehavior, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, DeleteResult, QueryTrait, ConnectionTrait};
use uuid::Uuid;

pub async fn find(
    db: &DatabaseConnection,
    id: Uuid,
) -> Option<roles::Model> {
    roles::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap()
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
    let role = roles::Model {
        id: Uuid::new_v4(),
        code: code.to_string(),
        name: name.to_string(),
    };

    let query = roles::Entity::insert(roles::ActiveModel::from(
        role.clone()
    ));

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await?;

    Ok(role)
}

pub async fn update<N: ToString>(
    db: &DatabaseConnection,
    id: Uuid,
    name: N,
) -> Result<roles::Model, DbErr> {
    let mut role = roles::ActiveModel::new();

    role.id = Set(id);
    role.name = Set(name.to_string());
    
    let query = roles::Entity::update(role);

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await
}

pub async fn delete(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<DeleteResult, DbErr> {
    let query = roles::Entity::delete_by_id(id);

    log::debug!(update, "{}", query.build(db.get_database_backend()));

    query.exec(db).await
}
