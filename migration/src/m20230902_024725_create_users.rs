use sea_orm_migration::{prelude::*, sea_orm::ActiveModelTrait};

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
                    "CREATE TABLE IF NOT EXISTS `users` (
                        `id` CHAR(36) NOT NULL PRIMARY KEY,
                        `name` VARCHAR(255) NOT NULL,
                        `email` VARCHAR(255) NOT NULL,
                        `email_verified_at` TIMESTAMP NULL DEFAULT NULL,
                        `username` VARCHAR(255) NOT NULL,
                        `password` VARCHAR(255) NOT NULL,
                        `profile_photo_id` CHAR(36) NULL DEFAULT NULL,
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        `deleted_at` TIMESTAMP NULL DEFAULT NULL
                    )"
                )
                .await?;
        } else {
            manager.create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(User::EmailVerifiedAt)
                            .timestamp()
                            .null()
                            .default(None as Option<String>)
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::ProfilePhotoId)
                            .string()
                            .null()
                            .default(None as Option<String>)
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()")
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()")
                    )
                    .col(
                        ColumnDef::new(User::DeletedAt)
                            .timestamp()
                            .null()
                            .default(None as Option<String>)
                    )
                    .take(),
            ).await?;
        }

        manager.create_index(
            Index::create()
                .table(User::Table)
                .name("idx_users_email")
                .col(User::Email)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(User::Table)
                .name("idx_users_email_verified_at")
                .col(User::EmailVerifiedAt)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(User::Table)
                .name("idx_users_username")
                .col(User::Username)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(User::Table)
                .name("idx_users_created_at")
                .col(User::CreatedAt)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(User::Table)
                .name("idx_users_updated_at")
                .col(User::UpdatedAt)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(User::Table)
                .name("idx_users_deleted_at")
                .col(User::DeletedAt)
                .to_owned()
        ).await?;

        let id = uuid::Uuid::new_v4();
        let name = "root".to_owned();
        let email = "root@local.app".to_owned();
        let email_verified_at = Some(chrono::Utc::now().naive_local());
        let username = "root".to_owned();
        let password = nightmare_common::hash::make(id.clone(), "LetMe!nM4te").to_string();
        let profile_photo_id: Option<String> = None;
        let created_at = chrono::Utc::now().naive_local();
        let updated_at = created_at.clone();
        let deleted_at = None;
        let user = nightmare_common::models::users::Model {
            id, name, email, email_verified_at, username, password,
            profile_photo_id, created_at, updated_at, deleted_at
        };

        nightmare_common::models::users::ActiveModel::from(user).insert(manager.get_connection()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).take()).await
    }
}

#[derive(DeriveIden)]
pub enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Name,
    Email,
    EmailVerifiedAt,
    Username,
    Password,
    ProfilePhotoId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
