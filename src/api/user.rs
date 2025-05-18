use actix_web::{ HttpResponse, Responder };

pub async fn user_login() -> impl Responder {
    HttpResponse::Ok().body("logged in")
}

pub async fn user_signup() -> impl Responder {
    HttpResponse::Ok().body("signed up")
}