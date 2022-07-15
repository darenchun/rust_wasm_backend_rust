use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Serialize;


//serde를 사용한 json to string serialization
#[derive(Serialize)]
struct ReturningJson {
    json_01: String,
}

#[get("/")]
async fn index() -> impl Responder {
    return HttpResponse::Ok().body("main_page");
}

pub async fn hello() ->  Result<impl Responder> {
    println!("hello has been activated");
    // struct를 json 타입 변환 후 serialization 을 자동으로 시행
    let returned_json : ReturningJson = ReturningJson { json_01: String::from("String_data") };
    return Ok(web::Json(returned_json));
}



