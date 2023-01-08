use actix_web::{
    body::BoxBody, get, http::header::ContentType, HttpRequest, HttpResponse, Responder,
  
};
use serde::Serialize;

//serde를 사용한 json to string serialization
#[derive(Serialize)]
struct ReturningJson {
    json_01: String,
}

impl Responder for ReturningJson {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    return HttpResponse::Ok().body("main_page");
}

pub async fn hello() -> impl Responder {
    println!("hello!!!!!!!");
    return ReturningJson {
        json_01: "json data".to_string(),
    };
}
