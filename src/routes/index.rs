//Configure handler
use actix_web::{Responder, HttpResponse};

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}