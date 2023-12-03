use sea_orm_migration::prelude::*;

use crate::{m20230902_024725_create_users::User, m20230902_024928_create_permissions::Permission};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
                Table::create()
                .table(PermissionUser::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(PermissionUser::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .extra("DEFAULT uuid_generate_v4()"),
                )
                .col(
                    ColumnDef::new(PermissionUser::PermissionId)
                        .uuid()
                        .not_null()
                )
                .col(
                    ColumnDef::new(PermissionUser::UserId)
                        .uuid()
                        .not_null()
                )
                .to_owned(),
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_permission_user_permission_id")
                .from(PermissionUser::Table, PermissionUser::PermissionId)
                .to(Permission::Table, Permission::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_permission_user_user_id")
                .from(PermissionUser::Table, PermissionUser::UserId)
                .to(User::Table, User::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(PermissionUser::Table).to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum PermissionUser {
    #[sea_orm(iden = "permission_user")]
    Table,
    Id,
    PermissionId,
    UserId,
}
