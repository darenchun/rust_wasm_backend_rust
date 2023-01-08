mod rest_api;
use rest_api::rest_get; // for example
mod repository; // for connection to db server
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use actix_cors::Cors;
use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*connections to db server*/
    repository::ddb::activate_connection_mysql();

    /* **run web application** */
    return HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .service(rest_get::index) // application boundaries
            .route("/manual_hello", web::get().to(manual_hello))
            .route("/hello", web::get().to(rest_get::hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await;
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
