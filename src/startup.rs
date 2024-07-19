use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use tracing_actix_web::TracingLogger;

use crate::{
    configuration::Configuration,
    routes::{healthcheck, home},
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

        let server = run(listener, configuration.application.base_url).await?;

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

async fn run(listener: TcpListener, base_url: String) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(base_url.clone())
            .route("/", web::get().to(home))
            .route("/healthcheck", web::get().to(healthcheck))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
