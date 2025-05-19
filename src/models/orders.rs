use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::orders;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(belongs_to(crate::models::orders::Order))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub price: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub price: i32
}