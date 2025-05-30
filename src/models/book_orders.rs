use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::book_orders;

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(crate::models::book_orders::BookOrder))]
#[diesel(table_name = book_orders)]
pub struct BookOrder {
    pub order_id: i32,
    pub book_id: i32,
    pub quantity: i32
}