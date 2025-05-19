use actix_web::{ HttpResponse, Responder };

pub async fn place_order() -> impl Responder {
    HttpResponse::Ok().body("place order")
}

pub async fn get_orders_by_user_id() -> impl Responder {
    HttpResponse::Ok().body("get orders by user id")
}