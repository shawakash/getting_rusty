use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::types::utils::ResponseStatus;

async fn get_client() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Get client",
        "status": ResponseStatus::Ok.as_str(),
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/client", web::get().to(get_client)));
}
