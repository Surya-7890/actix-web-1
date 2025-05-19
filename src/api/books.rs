use actix_web::{ web, HttpResponse, Responder };

use crate::models::books::{Book, NewBook, UpdateBook};
use crate::db_conn::DBPool;

pub async fn add_book(new_book: web::Json<NewBook>, db: web::Data<DBPool>) -> impl Responder {
    let book = new_book.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        Book::add_book(conn, &book)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn add_books(new_books: web::Json<Vec<NewBook>>, db: web::Data<DBPool>) -> impl Responder {
    let books = new_books.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        Book::add_books(conn, books)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn update_book_by_book_id(id: web::Path<i32>, updated_book: web::Json<UpdateBook>, db: web::Data<DBPool>) -> impl Responder {
    let book_id = id.into_inner();
    let book = updated_book.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        Book::update_book_by_book_id(conn, book_id, &book)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn get_all_books(db: web::Data<DBPool>) -> impl Responder {
    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        Book::get_all_books(conn)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn get_book_by_name(name: web::Path<String>, db: web::Data<DBPool>) -> impl Responder {
    let book_name = name.into_inner();
    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        Book::get_book_by_name(conn, &book_name)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn get_books_by_author_name(author: web::Path<String>, db: web::Data<DBPool>) -> impl Responder {
    let author_name = author.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        Book::get_books_by_author_name(conn, &author_name)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
