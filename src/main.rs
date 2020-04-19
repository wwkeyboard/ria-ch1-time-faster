use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use serde::Serialize;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "<h1>Hello, Rust in Action!</h1>
  <p>What is the <a href=\"/now\">time</a>?</p>",
    )
}

#[derive(Serialize)]
struct Timestamp {
    t: String,
}

#[get("/now")]
async fn now() -> impl Responder {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp {
        t: now.to_rfc3339(),
    };
    HttpResponse::Ok().json(timestamp)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(now))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
