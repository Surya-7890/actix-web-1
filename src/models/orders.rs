use diesel::prelude::*;
use crate::schema::orders;

#[derive(Debug, Queryable, Selectable)]
#[diesel(belongs_to(crate::models::orders::Order))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub price: i32
}

#[derive(Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub price: i32
}