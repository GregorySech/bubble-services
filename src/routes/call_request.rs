use actix_web::{
    http::{header::LOCATION, StatusCode},
    web, HttpResponse, Responder, ResponseError,
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama_actix::Template;
use chrono::Utc;
use serde::Deserialize;
use sqlx::{types::Uuid, PgPool};
use tracing::instrument;

use crate::domain::call_request::{CallRequestContactName, CallRequestPhoneNumber, NewCallRequest};

use super::error_chain_fmt;

#[derive(Template)]
#[template(path = "call_request.html")]
struct CallRequestTemplate {
    messages: Vec<FlashMessage>,
}

#[instrument(name = "Call Request page", skip(messages), fields(num_messages))]
pub async fn get(messages: IncomingFlashMessages) -> impl Responder {
    let messages: Vec<FlashMessage> = messages.iter().cloned().collect();
    tracing::Span::current().record("num_messages", messages.len());
    CallRequestTemplate { messages }
}

/// Raw call request input that needs to be parsed.
#[derive(Deserialize)]
pub struct CallRequestForm {
    phone_number: String,
    contact_name: String,
}

#[instrument(name = "Call Request submission", skip(form, pool))]
pub async fn post(
    form: web::Form<CallRequestForm>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CallRequestError> {
    let call_request =
        NewCallRequest::try_from(form.0).map_err(CallRequestError::ValidationError)?;
    let call_id = Uuid::new_v4();
    let created_at = Utc::now();

    sqlx::query!(
        r#"
            INSERT INTO call_requests (id, user_name, phone_number, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
        call_id,
        call_request.contact_name.as_ref(),
        call_request.phone_number.as_ref(),
        created_at
    )
    .execute(pool.get_ref())
    .await?;

    FlashMessage::info("Call request registered. You will be called soon!").send();
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

#[derive(thiserror::Error)]
pub enum CallRequestError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    InsertionError(#[from] sqlx::Error),
}

impl std::fmt::Debug for CallRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CallRequestError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let error_content = match self {
            CallRequestError::ValidationError(s) => s,
            CallRequestError::InsertionError(_) => "Database error!",
        };
        FlashMessage::error(error_content).send();
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/call_request"))
            .finish()
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CallRequestError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CallRequestError::InsertionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl TryFrom<CallRequestForm> for NewCallRequest {
    type Error = String;

    fn try_from(value: CallRequestForm) -> Result<Self, Self::Error> {
        let contact_name = CallRequestContactName::parse(value.contact_name)?;
        let phone_number = CallRequestPhoneNumber::parse(value.phone_number)?;

        Ok(NewCallRequest {
            phone_number,
            contact_name,
        })
    }
}
