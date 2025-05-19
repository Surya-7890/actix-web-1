use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::authors;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = authors)]
pub struct UpdateAuthor {
    pub name: String
}