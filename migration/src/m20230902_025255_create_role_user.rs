use sea_orm_migration::prelude::*;

use crate::{m20230902_025106_create_roles::Role, m20230902_024725_create_users::User};

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
                    "CREATE TABLE IF NOT EXISTS `role_user` (
                        `id` CHAR(36) NOT NULL PRIMARY KEY,
                        `role_id` CHAR(36) NOT NULL,
                        `user_id` CHAR(36) NOT NULL,
                        FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE,
                        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
                    )"
                )
                .await?;
        } else {
            manager.create_table(
                Table::create()
                    .table(RoleUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(RoleUser::RoleId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(RoleUser::UserId)
                            .uuid()
                            .not_null()
                    )
                    .to_owned(),
            ).await?;
            
            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_role_user_role_id")
                    .from(RoleUser::Table, RoleUser::RoleId)
                    .to(Role::Table, Role::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_role_user_user_id")
                    .from(RoleUser::Table, RoleUser::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(RoleUser::Table).to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum RoleUser {
    #[sea_orm(iden = "role_user")]
    Table,
    Id,
    RoleId,
    UserId,
}
