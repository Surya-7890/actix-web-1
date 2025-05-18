use actix_web::{ HttpResponse, Responder };

async fn get_all_books() -> impl Responder {
    HttpResponse::Ok().body("all books")
}

async fn get_book_by_name() -> impl Responder {
    HttpResponse::Ok().body("book by name")
}

async fn get_books_by_author() -> impl Responder {
    HttpResponse::Ok().body("books by author")
}

async fn get_books_by_category() -> impl Responder {
    HttpResponse::Ok().body("books by category")
}