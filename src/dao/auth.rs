use chrono::NaiveDateTime;
use nightmare_common::log;
use nightmare_common::models::{users, tokens};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryTrait, ConnectionTrait, QueryFilter, ColumnTrait, DeleteResult};
use uuid::Uuid;

pub async fn generate(
    db: &DatabaseConnection,
    user: &users::Model,
    expired_at: Option<NaiveDateTime>,
) -> Result<tokens::Model, DbErr> {
    let token = tokens::Model {
        id: Uuid::new_v4(),
        user_id: user.id,
        expired_at,
    };

    let query = tokens::Entity::insert(tokens::ActiveModel::from(token.clone()));

    log::debug!(generate, "{}", query.build(db.get_database_backend()));
    
    match query.exec(db).await {
        Err(e) => {
            log::error!(generate, "{}", e);

            Err(e)
        },
        Ok(_) => Ok(token),
    }
}

pub async fn delete(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<DeleteResult, DbErr> {
    tokens::Entity::delete_many()
        .filter(tokens::Column::UserId.eq(user_id))
        .exec(db)
        .await
}
