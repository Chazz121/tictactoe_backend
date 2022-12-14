use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
mod routes;
use routes::register;
mod game;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
