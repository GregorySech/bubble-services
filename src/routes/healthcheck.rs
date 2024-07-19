use actix_web::Responder;

pub async fn healthcheck() -> impl Responder {
    "OK"
}
