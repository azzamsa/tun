pub mod create_user;
pub mod delete_user;
pub mod find_user;
pub mod find_users;
pub mod get_meta;
pub mod get_zen;
pub mod update_user;

use crate::{drivers::github::Github, repository::Repository};

pub struct Service {
    repo: Repository,
    github: Github,
}

impl Service {
    pub fn new(repo: Repository, github: Github) -> Self {
        Self { repo, github }
    }
}
