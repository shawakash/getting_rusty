use actix_web::{web, HttpResponse, HttpServer, Responder};
use serde_json::json;

use crate::types::utils::ResponseStatus;

async fn binance() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Binance API",
        "status": ResponseStatus::BadRequest.as_str(),
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/binance")
            .route("/account", web::get().to(binance))
            .route("/orders", web::get().to(binance))
            .route("/order", web::post().to(binance))
            .route("/order", web::delete().to(binance))
            .route("/order", web::put().to(binance))
            .route("/order", web::patch().to(binance))
            .route("/order", web::head().to(binance))
            .route("/order", web::trace().to(binance)),
    );
}
