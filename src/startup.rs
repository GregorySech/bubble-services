use std::net::TcpListener;

use actix_web::{cookie::Key, dev::Server, web, App, HttpServer};

use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{Configuration, DatabaseConfiguration},
    routes::{call_request, healthcheck, home, login},
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    #[tracing::instrument(name = "Building application from configuration")]
    pub async fn build(configuration: Configuration) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();

        let server = run(
            listener,
            configuration.application.base_url,
            make_database_pool(&configuration.database),
            configuration.application.hmac_secret,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    base_url: String,
    db_pool: PgPool,
    hmac_secret: Secret<String>,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_backend = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_backend).build();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(TracingLogger::default())
            .app_data(base_url.clone())
            .app_data(db_pool.clone())
            .route("/", web::get().to(home))
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/call_request", web::get().to(call_request::get))
            .route("/call_request", web::post().to(call_request::post))
            .route("/login", web::get().to(login::get))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn make_database_pool(configuration: &DatabaseConfiguration) -> Pool<Postgres> {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
