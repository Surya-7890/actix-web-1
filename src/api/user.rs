use actix_web::{ web, HttpResponse, Responder };
use crate::db_conn::DBPool;

pub async fn user_signup(db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("user signup")
}

pub async fn user_login(db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("user login")
}

pub async fn update_user(db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("update user")
}

pub async fn delete_user(db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("delete user")
}