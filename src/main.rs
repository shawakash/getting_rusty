use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::io::Result;
use std::time::{Duration, SystemTime};

mod config;
mod routes;
mod types;
use crate::routes::binance::binance;
use crate::types::utils::ResponseStatus;

static START_TIME: SystemTime = SystemTime::UNIX_EPOCH;

#[get("/")]
async fn main_handler() -> impl Responder {
    let uptime = SystemTime::now()
        .duration_since(START_TIME)
        .expect("Time went backwards")
        .as_secs();
    HttpResponse::Ok().json(json!({
        "uptime": uptime,
        "message": "Server is healthy!",
        "status": ResponseStatus::Ok.as_str()
    }))
}

#[get("/_health")]
async fn health() -> impl Responder {
    let uptime = SystemTime::now()
        .duration_since(START_TIME)
        .expect("Time went backwards")
        .as_secs();
    HttpResponse::Ok().json(json!({
        "uptime": uptime,
        "message": "Server is healthy!",
        "status": ResponseStatus::Ok.as_str()
    }))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let port = config::server_port();
    println!("Server running on port: {}", port);

    HttpServer::new(|| {
        App::new().service(main_handler).service(health).service(
            web::scope("/app")
                .route("/binance", web::get().to(binance))
                .route("/hey", web::get().to(manual_hello)),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
