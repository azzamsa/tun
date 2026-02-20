use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::users;

#[derive(ToSchema, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(ToSchema, Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
#[derive(ToSchema, Deserialize, Serialize)]
pub struct UpdateUser {
    pub name: String,
    pub full_name: Option<String>,
}
