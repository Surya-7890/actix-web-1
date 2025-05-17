use actix_web::{ get, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("hello")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(home)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}