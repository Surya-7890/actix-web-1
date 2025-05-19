use actix_web::{ web, HttpResponse, Responder };

use crate::models::books::{NewBook, UpdateBook};

pub async fn add_book(new_book: web::Json<NewBook>) -> impl Responder {
    HttpResponse::Ok().body("add book")
}

pub async fn add_books(new_books: web::Json<Vec<NewBook>>) -> impl Responder {
    HttpResponse::Ok().body("add books")
}

pub async fn update_book_by_book_id(id: web::Path<i32>, updated_book: web::Json<UpdateBook>) -> impl Responder {
    HttpResponse::Ok().body("update book")
}

pub async fn get_all_books() -> impl Responder {
    HttpResponse::Ok().body("get all books")
}

pub async fn get_book_by_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("get book by name")
}

pub async fn get_books_by_author_id(author_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body("get all books by an author")
}
