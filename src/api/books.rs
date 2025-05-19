use actix_web::{ HttpResponse, Responder };

pub async fn add_book() -> impl Responder {
    HttpResponse::Ok().body("add book")
}

pub async fn add_books() -> impl Responder {
    HttpResponse::Ok().body("add books")
}

pub async fn update_book_by_book_id() -> impl Responder {
    HttpResponse::Ok().body("update book")
}

pub async fn get_all_books() -> impl Responder {
    HttpResponse::Ok().body("get all books")
}

pub async fn get_book_by_name() -> impl Responder {
    HttpResponse::Ok().body("get book by name")
}

pub async fn get_books_by_author_id() -> impl Responder {
    HttpResponse::Ok().body("get all books by an author")
}
