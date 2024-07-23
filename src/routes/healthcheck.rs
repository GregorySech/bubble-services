use actix_web::Responder;

#[tracing::instrument]
pub async fn healthcheck() -> impl Responder {
    "OK"
}
