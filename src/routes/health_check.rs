use actix_web::{HttpResponse, Responder};

pub async fn health_checker() -> impl Responder {
    HttpResponse::Ok()
}
