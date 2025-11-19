use entity::user;
use sea_orm as orm;
use sea_orm::EntityTrait;

pub async fn users(db: &orm::DatabaseConnection) -> Result<Vec<user::Model>, crate::Error> {
    let users: Vec<user::Model> = user::Entity::find().all(db).await?;
    Ok(users)
}
