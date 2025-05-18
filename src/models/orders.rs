use diesel::prelude::*;
use crate::schema::orders;

#[derive(Debug, Queryable, Selectable)]
#[diesel(belongs_to(crate::models::orders::Order))]
#[diesel(table_name = orders)]
pub struct Order {
    id: i32,
    user_id: i32,
    price: i32
}