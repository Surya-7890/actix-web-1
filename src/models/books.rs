use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::books;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(belongs_to(crate::models::author::Author))]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub author: String,
    pub price: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub year: i32,
    pub author: String,
    pub price: i32
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = books)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub year: Option<i32>,
    pub author: Option<String>,
    pub price: Option<i32>
}