use actix_web::{ HttpResponse, Responder };

pub async fn add_author() -> impl Responder {
    HttpResponse::Ok().body("add author")
}

pub async fn update_author() -> impl Responder {
    HttpResponse::Ok().body("update author")
}