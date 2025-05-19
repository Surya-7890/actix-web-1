use crate::models::books::{ Book, NewBook, UpdateBook };
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use crate::schema::books::dsl::*;

impl Book {
    pub fn add_book(conn: &mut PgConnection, new_book: &NewBook) -> Result<Book, diesel::result::Error> {
        diesel::insert_into(books).values(new_book).get_result(conn)
    }

    pub fn add_books(conn: &mut PgConnection, new_books: Vec<&NewBook>) -> Result<Vec<Book>, diesel::result::Error> {
        diesel::insert_into(books).values(new_books).get_results(conn)
    }

    pub fn update_book_by_book_id(conn: &mut PgConnection, book_id: i32, updated_book: &UpdateBook) -> Result<Book, diesel::result::Error> {
        diesel::update(books.filter(id.eq(book_id)))
            .set(updated_book)
            .get_result(conn)
    }
    
    pub fn get_all_books(conn: &mut PgConnection) -> Result<Vec<Book>, diesel::result::Error> {
        books.load::<Book>(conn)
    }
    
    pub fn get_book_by_name(conn: &mut PgConnection, book_name: &str) -> Result<Book, diesel::result::Error> {
        books.filter(title.eq(book_name.to_string())).first::<Book>(conn)
    }
    
    pub fn get_books_by_author_id(conn: &mut PgConnection, author_id_val: i32) -> Result<Vec<Book>, diesel::result::Error> {
        books.filter(author_id.eq(author_id_val)).load::<Book>(conn)
    }
}