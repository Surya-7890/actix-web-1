use actix_web::{ web, HttpResponse, Responder };
use serde::Deserialize;
use crate::db_conn::DBPool;

#[derive(Deserialize)]
pub struct PlaceOrderRequest {
    pub user_id: i32,
    pub books: Vec<i32>
}

pub async fn place_order(order: web::Json<PlaceOrderRequest>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("place order")
}

pub async fn get_orders_by_user_id(user_id: web::Path<i32>, db: web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body("get orders by user id")
}