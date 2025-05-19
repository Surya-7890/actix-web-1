use diesel::prelude::*;
use crate::schema::authors;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String
}

#[derive(AsChangeset)]
#[diesel(table_name = authors)]
pub struct UpdateAuthor {
    pub name: String
}