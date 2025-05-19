use actix_web::{ HttpResponse, Responder };

pub async fn user_signup() -> impl Responder {
    HttpResponse::Ok().body("user signup")
}

pub async fn user_login() -> impl Responder {
    HttpResponse::Ok().body("user login")
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("update user")
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete user")
}