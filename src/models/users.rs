use diesel::prelude::*;
use crate::schema::users;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(belongs_to(crate::models::users::Users))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>
}