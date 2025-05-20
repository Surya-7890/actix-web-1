use actix_web::{ web, HttpResponse, Responder };
use serde::Deserialize;
use crate::db_conn::DBPool;
use crate::models::orders::UpdateOrder;
use crate::models::{ book_orders::BookOrder, orders::{Order, NewOrder}, books::Book };

#[derive(Deserialize)]
pub struct SingleBookOrder {
    pub id: i32,
    pub quantity: i32
}

#[derive(Deserialize)]
pub struct PlaceOrderRequest {
    pub user_id: i32,
    pub books: Vec<SingleBookOrder>
}

pub async fn place_order(order: web::Json<PlaceOrderRequest>, db: web::Data<DBPool>) -> impl Responder {
    let orders = order.into_inner();
    let user_id_val = orders.user_id;
    let conn = &mut db.get().unwrap();

    let new_order = Order::place_order(conn, &NewOrder { 
        user_id: user_id_val, 
        price: 0 
    });

    match new_order {
        Ok(order) => {
            let mut total_price = 0;
            for book in orders.books {
                let book_res = Book::get_book_by_book_id(conn, book.id);
        
                match book_res {
                    Ok(data) => {
                        let book_order = BookOrder::place_book_order(conn, &BookOrder { 
                            order_id: order.id, 
                            book_id: data.id, 
                            quantity: book.quantity
                        });

                        match book_order {
                            Ok(_) => {
                                total_price += data.price * book.quantity;
                            }
                            Err(err) => {
                                return HttpResponse::InternalServerError().body(err.to_string())
                            }
                        }
                    }
                    Err(err) => {
                        return HttpResponse::InternalServerError().body(err.to_string())
                    }
                }
        
            }

            let updated_order = Order::update_order_by_order_id(conn, order.id, &UpdateOrder{
                user_id: Some(order.user_id),
                price: Some(total_price)
            });

            match updated_order {
                Ok(data) => return HttpResponse::Created().json(data),
                Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_orders_by_order_id(order_id: web::Path<i32>, db: web::Data<DBPool>) -> impl Responder {
    let order_id_val = order_id.into_inner();

    let result = web::block(move|| {
        let conn = &mut db.get().unwrap();
        BookOrder::get_all_book_orders_by_order_id(conn, order_id_val)
    }).await;

    match result {
        Ok(Ok(data)) => HttpResponse::Created().json(data),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}