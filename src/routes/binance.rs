use actix_web::{web, HttpResponse, HttpServer, Responder};
use serde_json::json;

use crate::types::utils::ResponseStatus;

pub async fn binance() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Binance API",
        "status": ResponseStatus::BadRequest.as_str(),
    }))
}
