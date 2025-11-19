use sea_orm::entity::*;
use sea_orm_migration::prelude::*;

use entity::{prelude::*, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let user = user::ActiveModel {
            name: Set("Aragorn".to_owned()),
            full_name: Set(Some("Aragorn Telcontar".to_owned())),
            ..Default::default()
        };
        User::insert(user).exec(db).await?;

        let user = user::ActiveModel {
            // full name is optional
            name: Set("Gandalf".to_owned()),
            ..Default::default()
        };
        User::insert(user).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        User::delete_many().exec(db).await?;
        Ok(())
    }
}
