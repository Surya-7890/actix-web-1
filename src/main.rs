use actix_web::{ get, App, HttpResponse, HttpServer, Responder, web };
mod api;
mod db;
mod models;
mod schema;

use api::{ user };

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("hello")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(
                web::scope("/user")
                    .route("/login", web::post().to(user::user_login))
                    .route("/signup", web::post().to(user::user_signup))
            )
            // .service(factory)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}