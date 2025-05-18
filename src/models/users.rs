use diesel::prelude::*;
use crate::schema::users;

#[derive(Debug, Queryable, Selectable)]
#[diesel(belongs_to(crate::models::users::Users))]
#[diesel(table_name = users)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String
}