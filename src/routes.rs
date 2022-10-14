use super::game::register_routes;
use actix_web::{dev::HttpServiceFactory, get, web, App, HttpResponse, Responder, Scope};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn register() -> Scope {
    web::scope("")
        .service(hello)
        .service(web::scope("game").service(register_routes()))
}
