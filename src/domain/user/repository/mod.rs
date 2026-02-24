use crate::db::Db;

pub mod create_user;
pub mod delete_user;
pub mod find_user;
pub mod find_users;
pub mod update_user;

pub struct Repository {
    db: Db,
}

impl Repository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
