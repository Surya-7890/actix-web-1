use actix_web::{ web, HttpResponse, Responder };
use serde::Deserialize;
use crate::db_conn::DBPool;
use crate::models::users::{NewUser, UpdateUser, User};

pub async fn user_signup(user: web::Json<NewUser>, db: web::Data<DBPool>) -> impl Responder {
    let new_user = user.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        User::user_signup(conn, &new_user)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String
}

pub async fn user_login(user: web::Json<UserLoginRequest>, db: web::Data<DBPool>) -> impl Responder {
    let user_info = user.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        User::user_login(conn, &user_info.username, &user_info.password)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn update_user(user_id: web::Path<i32>, updated_user: web::Json<UpdateUser>,db: web::Data<DBPool>) -> impl Responder {
    let user = updated_user.into_inner();
    let user_id_val = user_id.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        User::update_user(conn, user_id_val, &user)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn delete_user(user_id: web::Path<i32>, db: web::Data<DBPool>) -> impl Responder {
    let user_id_val = user_id.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        User::delete_user(conn, user_id_val)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}