mod rest_api;
use rest_api::rest_get; // for example
mod repository; // for connection to db server
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    /*connections to db server*/ 
    repository::ddb::activate_connection_mysql();

    /* **run web application** */
    return HttpServer::new(|| {
        App::new()
            .service(rest_get::index) // application boundaries
            .route("/", web::get().to(manual_hello))
            .route("/articles", web::get().to(rest_get::hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await;
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
