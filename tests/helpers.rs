use bubble_services::{
    configuration::{get_configuration, DatabaseConfiguration},
    startup::{make_database_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use reqwest::{Response, Url};
use serde::Serialize;
use sqlx::{types::Uuid, ConnectOptions, Connection, Executor, PgConnection, PgPool};
// Set's up telemetry once.
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
});

/// Checks for correct response configuration for redirects.
pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

/// Test deployment of the application.
pub struct TestApp {
    pub address: String,
    pub base_url: Url,
    pub db_pool: PgPool,
    pub http_client: reqwest::Client,
    pub port: u16,
}

/// Creates a database according to the provided settings using the project's migrations.
async fn configure_database(config: &DatabaseConfiguration) -> PgPool {
    let connection_options = config
        .without_db()
        .log_statements(tracing_log::log::LevelFilter::Trace);
    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres DB!");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

impl TestApp {
    /// Spawn the application for testing.
    pub async fn spawn() -> TestApp {
        // Setting up telemetry
        Lazy::force(&TRACING);

        // Getting configuration.
        let configuration = {
            let mut c = get_configuration().expect("Failed to load configuration.");

            c.database.database_name = format!("bubble_services_test_{}", Uuid::new_v4());
            c.application.port = 0; // Connect to a free port!
            c
        };

        configure_database(&configuration.database).await;

        let app = Application::build(configuration.clone())
            .await
            .expect("Failed to build the application server");
        let application_port = app.port();
        let address = format!("http://127.0.0.1:{}", application_port);

        // Spawn application.
        tokio::spawn(app.run_until_stopped());

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .unwrap();

        TestApp {
            address,
            db_pool: make_database_pool(&configuration.database),
            http_client: client,
            base_url: Url::parse(&configuration.application.base_url).unwrap(),
            port: application_port,
        }
    }

    pub async fn get_home_page(&self) -> Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to get website home.")
    }

    pub async fn get_call_request_page(&self) -> Response {
        self.http_client
            .get(format!("{}/call_request", &self.address))
            .send()
            .await
            .expect("Failed to get call request page.")
    }

    pub async fn post_call_request<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}/call_request", &self.address))
            .form(body)
            .send()
            .await
            .expect("Could not post call request form!")
    }
}
