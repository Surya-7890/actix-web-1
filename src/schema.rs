// @generated automatically by Diesel CLI.

diesel::table! {
    book_orders (order_id, book_id) {
        order_id -> Int4,
        book_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        year -> Int4,
        author -> Varchar,
        price -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        price -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(book_orders -> books (book_id));
diesel::joinable!(book_orders -> orders (order_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    book_orders,
    books,
    orders,
    users,
);
