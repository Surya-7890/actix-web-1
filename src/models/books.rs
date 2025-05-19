use diesel::prelude::*;
use crate::schema::books;

#[derive(Debug, Queryable, Selectable)]
#[diesel(belongs_to(crate::models::author::Author))]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub author_id: i32,
    pub price: i32
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub year: i32,
    pub author_id: i32,
    pub price: i32
}

#[derive(AsChangeset)]
#[diesel(table_name = books)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub year: Option<i32>,
    pub author_id: Option<i32>,
    pub price: Option<i32>
}