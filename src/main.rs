use actix_web::{ get, App, HttpResponse, HttpServer, Responder, web };
mod api;
mod db;
mod models;
mod schema;
mod db_conn;

use api::{ user, books, orders };

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("hello")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let conn_pool = db_conn::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn_pool.clone()))
            .service(home)
            .service(
                web::scope("/user")
                    .route("/{id}", web::patch().to(user::update_user))
                    .route("/{id}", web::delete().to(user::delete_user))
                    .route("/login", web::post().to(user::user_login))
                    .route("/signup", web::post().to(user::user_signup))
            )
            .service(
                web::scope("/books")
                    .route("/", web::get().to(books::get_all_books))
                    .route("/{id}", web::patch().to(books::update_book_by_book_id))
                    .route("/add", web::post().to(books::add_book))
                    .route("/add/multiple", web::post().to(books::add_books))
                    .route("/name/{name}", web::get().to(books::get_book_by_name))
                    .route("/author/{author_id}", web::get().to(books::get_books_by_author_name))
            )
            .service(
                web::scope("/orders")
                    .route("/", web::post().to(orders::place_order))
                    .route("/{order_id}", web::get().to(orders::get_orders_by_order_id))
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}