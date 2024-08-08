//! # Home page
//! The home page contains links to all the actions that the user can perform.
//! The user actions depend on their authorizations.
//!
//! ## Unauthenticated actions
//! Call requests can be submitted also by users that are not authenticated.
//! Unauthenticated users can be authenticated using the login action.

use actix_web::Responder;
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    messages: Vec<FlashMessage>,
}

#[tracing::instrument(name = "Home", skip(messages))]
pub async fn home(messages: IncomingFlashMessages) -> impl Responder {
    let messages: Vec<FlashMessage> = messages.iter().cloned().collect();

    HomeTemplate { messages }
}
