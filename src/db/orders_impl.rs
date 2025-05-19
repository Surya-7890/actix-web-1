use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::orders::{NewOrder, Order};
use crate::schema::orders::dsl::*;

impl Order {
    pub fn place_order(conn: &mut PgConnection, new_order: &NewOrder) -> Result<Order, diesel::result::Error> {
        diesel::insert_into(orders)
            .values(new_order)
            .get_result(conn)
    }

    pub fn get_orders_by_user_id(conn: &mut PgConnection, user_id_val: i32) -> Result<Vec<Order>, diesel::result::Error> {
        orders.filter(user_id.eq(user_id_val))
            .load::<Order>(conn)
    }
}