use sea_orm_migration::prelude::*;

use crate::{m20230902_024928_create_permissions::Permission, m20230902_025106_create_roles::Role};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
                Table::create()
                .table(PermissionRole::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(PermissionRole::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .extra("DEFAULT uuid_generate_v4()"),
                )
                .col(
                    ColumnDef::new(PermissionRole::PermissionId)
                        .uuid()
                        .not_null()
                )
                .col(
                    ColumnDef::new(PermissionRole::RoleId)
                        .uuid()
                        .not_null()
                )
                .to_owned(),
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_permission_role_permission_id")
                .from(PermissionRole::Table, PermissionRole::PermissionId)
                .to(Permission::Table, Permission::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_permission_role_user_id")
                .from(PermissionRole::Table, PermissionRole::RoleId)
                .to(Role::Table, Role::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(PermissionRole::Table).to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum PermissionRole {
    #[sea_orm(iden = "permission_role")]
    Table,
    Id,
    PermissionId,
    RoleId,
}