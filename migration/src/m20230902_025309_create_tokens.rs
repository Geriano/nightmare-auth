use sea_orm_migration::prelude::*;

use crate::m20230902_024725_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let is_postgres = url.starts_with("postgres://");

        if !is_postgres {
            manager.get_connection()
                .execute_unprepared(
                    "CREATE TABLE IF NOT EXISTS `tokens` (
                        `id` CHAR(36) NOT NULL PRIMARY KEY,
                        `user_id` CHAR(36) NOT NULL,
                        `expired_at` TIMESTAMP NULL DEFAULT NULL,
                        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
                    )"
                )
                .await?;
        } else {
            manager.create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Token::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(Token::UserId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Token::ExpiredAt)
                            .timestamp()
                            .null()
                            .default(None as Option<String>)
                    )
                    .to_owned(),
            ).await?;

            manager.create_index(
                Index::create()
                    .table(Token::Table)
                    .name("idx_tokens_user_id")
                    .col(Token::UserId)
                    .to_owned()
            ).await?;

            manager.create_index(
                Index::create()
                    .table(Token::Table)
                    .name("idx_tokens_expired_at")
                    .col(Token::ExpiredAt)
                    .to_owned()
            ).await?;

            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_tokens_user_id")
                    .from(Token::Table, Token::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Token::Table).to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Token {
    #[sea_orm(iden = "tokens")]
    Table,
    Id,
    UserId,
    ExpiredAt,
}
