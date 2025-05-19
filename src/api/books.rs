use actix_web::{ web, HttpResponse, Responder };

use crate::models::books::{NewBook, UpdateBook};
use crate::db_conn::DBPool;

pub async fn add_book(new_book: web::Json<NewBook>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("add book")
}

pub async fn add_books(new_books: web::Json<Vec<NewBook>>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("add books")
}

pub async fn update_book_by_book_id(id: web::Path<i32>, updated_book: web::Json<UpdateBook>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("update book")
}

pub async fn get_all_books(db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("get all books")
}

pub async fn get_book_by_name(name: web::Path<String>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("get book by name")
}

pub async fn get_books_by_author_id(author_id: web::Path<i32>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("get all books by an author")
}
