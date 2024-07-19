use bubble_services::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use reqwest::Url;

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
    pub http_client: reqwest::Client,
    pub port: u16,
}

impl TestApp {
    /// Spawn the application for testing.
    pub async fn spawn() -> TestApp {
        // Setting up telemetry
        Lazy::force(&TRACING);

        // Getting configuration.
        let configuration = {
            let mut c = get_configuration().expect("Failed to load configuration.");

            c.application.port = 0; // Connect to a free port!
            c
        };

        let app = Application::build(configuration.clone())
            .await
            .expect("Failed to build the application server");
        let application_port = app.port();
        let address = format!("http://127.0.0.1:{}", application_port);

        // Spawn application.
        tokio::spawn(app.run_until_stopped());

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        let test_app = TestApp {
            address,
            http_client: client,
            base_url: Url::parse(&configuration.application.base_url).unwrap(),
            port: application_port,
        };

        test_app
    }
}
