use actix_web::{ web, HttpResponse, Responder };

use crate::models::author::{NewAuthor, UpdateAuthor};

pub async fn add_author(author: web::Json<NewAuthor>) -> impl Responder {
    HttpResponse::Ok().body("add author")
}

pub async fn update_author(author: web::Json<UpdateAuthor>) -> impl Responder {
    HttpResponse::Ok().body("update author")
}