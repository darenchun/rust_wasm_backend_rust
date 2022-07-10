use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod rest_api;
use rest_api::rest_get;
// use rest_api::rest_post;
// use rest_api::rest_put;
// use rest_api::rest_del;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let first_scope = web::scope("/first_category");

    return HttpServer::new(|| {
        App::new()
            .service(rest_get::index,)// application boundaries
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
