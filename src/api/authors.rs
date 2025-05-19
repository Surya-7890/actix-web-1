use actix_web::{ web, HttpResponse, Responder };

use crate::{db_conn::DBPool, models::author::{NewAuthor, UpdateAuthor}};

pub async fn add_author(author: web::Json<NewAuthor>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("add author")
}

pub async fn update_author(author: web::Json<UpdateAuthor>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("update author")
}