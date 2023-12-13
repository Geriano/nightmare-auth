use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let is_postgres = url.starts_with("postgres://");

        if !is_postgres {
            // set foreign key on for sqlite
            manager.get_connection()
                .execute_unprepared("PRAGMA foreign_keys = ON")
                .await?;

            return Ok(());
        }

        manager.get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let is_postgres = url.starts_with("postgres://");

        if !is_postgres {
            return Ok(());
        }

        manager.get_connection()
            .execute_unprepared("DROP EXTENSION IF EXISTS \"uuid-ossp\"")
            .await?;

        Ok(())
    }
}
