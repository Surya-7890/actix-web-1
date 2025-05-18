use diesel::prelude::*;
use crate::schema::authors;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String
}