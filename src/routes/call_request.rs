use actix_web::Responder;
use askama_actix::Template;

#[derive(Template)]
#[template(path = "call_request.html")]
struct CallRequestTemplate {}

pub async fn get() -> impl Responder {
    CallRequestTemplate {}
}
