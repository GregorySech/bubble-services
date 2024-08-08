use actix_web::Responder;
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    messages: Vec<FlashMessage>,
}

#[tracing::instrument(name = "Login Form", skip(messages))]
pub async fn get(messages: IncomingFlashMessages) -> impl Responder {
    LoginTemplate {
        messages: messages.iter().cloned().collect(),
    }
}
