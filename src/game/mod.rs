use actix_web::{get, Responder, HttpResponse, web, Scope};
mod routes;

struct Board {
    marks: Vec<Vec<String>>
}

struct Game {
    board: Board
}

pub fn register_routes() -> Scope {
    web::scope("").service(routes::hello)
}