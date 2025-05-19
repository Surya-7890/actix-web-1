use actix_web::{ HttpResponse, Responder };

pub async fn get_all_books() -> impl Responder {
    HttpResponse::Ok().body("all books")
}

pub async fn get_book_by_name() -> impl Responder {
    HttpResponse::Ok().body("book by name")
}

pub async fn get_books_by_author() -> impl Responder {
    HttpResponse::Ok().body("books by author")
}
