use actix_web::Responder;
use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    name: &'a str,
}

pub async fn home() -> impl Responder {
    HomeTemplate { name: "User" }
}
