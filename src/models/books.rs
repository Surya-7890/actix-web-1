use diesel::prelude::*;
use crate::schema::books;

#[derive(Debug, Queryable, Selectable)]
#[diesel(belongs_to(crate::models::author::Author))]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub year: Option<i32>,
    pub author_id: Option<i32>
}