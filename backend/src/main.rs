use actix_web::{App, HttpServer, web::{Path, self, Json}, post, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct JsonReq {
    name: String,
    age: i8
}

#[derive(Serialize, Deserialize)]
struct JsonStruct {
    msg: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        let cors = actix_cors::Cors::permissive();
        App::new().wrap(cors).default_service(web::to(default_route)).route("/{num}", web::get().to(route_fn)).service(route_service)
    }).bind(("127.0.0.1", 8090)).unwrap().run().await
}

#[post("/json/req")]
async fn route_service(user: Json<JsonReq>) -> HttpResponse {
    let msg = format!("Hello, {} you're {} years old!", user.name, user.age);
    HttpResponse::Ok().json(JsonStruct {msg})
}

async fn route_fn(num: Path<i32>) -> impl Responder {
    format!("{}", num.to_owned() + 1)
}

async fn default_route() -> impl Responder {
    "Bruh"
}
