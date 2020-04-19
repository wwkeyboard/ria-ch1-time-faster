use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "<h1>Hello, Rust in Action!</h1>
  <p>What is the <a href=\"/now\">time</a>?</p>",
    )
}

#[get("/now")]
async fn now() -> impl Responder {}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
