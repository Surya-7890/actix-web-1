use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::book_orders::BookOrder;
use crate::schema::book_orders::dsl::*;

impl BookOrder {
    pub fn place_book_order(conn: &mut PgConnection, book_order: &BookOrder) -> Result<BookOrder, diesel::result::Error> {
        diesel::insert_into(book_orders)
            .values(book_order)
            .get_result(conn)
    }

    pub fn get_all_book_orders_by_order_id(conn: &mut PgConnection, order_id_val: i32) -> Result<Vec<BookOrder>, diesel::result::Error> {
        book_orders.filter(order_id.eq(order_id_val))
            .load(conn)
    }
}