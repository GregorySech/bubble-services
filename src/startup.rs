use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{Configuration, DatabaseConfiguration},
    routes::{call_request, healthcheck, home},
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
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(base_url.clone())
            .app_data(db_pool.clone())
            .route("/", web::get().to(home))
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/call_request", web::get().to(call_request::get))
            .route("/call_request", web::post().to(call_request::post))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn make_database_pool(configuration: &DatabaseConfiguration) -> Pool<Postgres> {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
