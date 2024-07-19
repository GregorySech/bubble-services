#![doc = include_str!("../README.md")]

use anyhow::Context;
use bubble_services::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("bubble_services".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().context("Could not get configuration")?;
    let app = Application::build(config)
        .await
        .context("Could not build application")?;

    app.run_until_stopped()
        .await
        .context("Could not run application")
}
