use diesel::prelude::*;
use crate::models::author::{Author, NewAuthor};
use crate::schema::authors::dsl::*;
use diesel::PgConnection;

impl Author {
    pub fn add_author(conn: &mut PgConnection, author_name: &str) -> Result<Author, diesel::result::Error> {
        let new_author = NewAuthor {
            name: author_name.to_string()
        };
        diesel::insert_into(authors).values(&new_author).get_result(conn)
    }

    pub fn update_author(conn: &mut PgConnection, author_id: i32, updated_author: &crate::models::author::UpdateAuthor) -> Result<Author, diesel::result::Error> {
        diesel::update(authors.filter(id.eq(author_id)))
            .set(updated_author)
            .get_result(conn)
    }
}