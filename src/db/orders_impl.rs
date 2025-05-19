use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::orders::{NewOrder, Order, UpdateOrder};
use crate::schema::orders::dsl::*;

impl Order {
    pub fn place_order(conn: &mut PgConnection, new_order: &NewOrder) -> Result<Order, diesel::result::Error> {
        diesel::insert_into(orders)
            .values(new_order)
            .get_result(conn)
    }

    pub fn update_order_by_order_id(conn: &mut PgConnection, order_id: i32, updated_order: &UpdateOrder) -> Result<Order, diesel::result::Error> {
        diesel::update(orders.filter(id.eq(order_id)))
            .set(updated_order)
            .get_result(conn)
    }
}