use sea_orm_migration::prelude::*;

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
                    "CREATE TABLE IF NOT EXISTS roles (
                        id VARCHAR(36) NOT NULL PRIMARY KEY,
                        code VARCHAR(255) NOT NULL UNIQUE,
                        name VARCHAR(255) NOT NULL
                    )"
                )
                .await?;
        } else {
            manager
                .create_table(
                    Table::create()
                        .table(Role::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Role::Id)
                                .uuid()
                                .not_null()
                                .primary_key()
                                .extra("DEFAULT uuid_generate_v4()"),
                        )
                        .col(
                            ColumnDef::new(Role::Code)
                                .string()
                                .not_null()
                                .unique_key()
                        )
                        .col(
                            ColumnDef::new(Role::Name)
                                .string()
                                .not_null()
                        )
                        .to_owned(),
                )
                .await?;
        }

        manager.create_index(
            Index::create()
                .table(Role::Table)
                .name("idx_roles_code")
                .col(Role::Code)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Role::Table)
                .name("idx_roles_name")
                .col(Role::Name)
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Role::Table).take()).await
    }
}

#[derive(DeriveIden)]
pub enum Role {
    #[sea_orm(iden = "roles")]
    Table,
    Id,
    Code,
    Name,
}
