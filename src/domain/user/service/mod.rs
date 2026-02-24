use crate::{db::Db, domain::user::repository::Repository};

pub mod create_user;
pub mod delete_user;
pub mod find_user;
pub mod find_users;
pub mod update_user;

pub struct Service {
    repo: Repository,
}

impl Service {
    pub fn new(db: Db) -> Self {
        let repo = Repository::new(db);
        Self { repo }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateProfileInput {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}
