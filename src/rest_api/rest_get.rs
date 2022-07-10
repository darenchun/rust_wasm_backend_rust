use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    return HttpResponse::Ok().body("main_page");
}

// #[get("/")]
// async fn index() -> impl Responder {
//     return HttpResponse::Ok().body("Hello world!");
// }

// Trait 가 뭔가를 하는거 같은데 handler를 retrun 하지 않는다.
// #[get("/articles")]
// async fn hello() -> impl Responder {
//     println!("Hello was initiated");
//     return HttpResponse::Ok().body("Hello articles!");
// }

pub async fn hello() -> impl Responder {
    println!("Hello was initiated");
    let returned_string : String = "Hello".to_owned();
    return HttpResponse::Ok().body(returned_string);
}


// async fn main_page() -> impl Responder {
//     return HttpResponse::Ok().body("Hello world!");
// }


