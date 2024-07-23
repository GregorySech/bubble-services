use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationConfiguration {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseConfiguration {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory. (Doesn't exists or not permitted)");
    let configuration_directory = base_path.join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(
            config::Environment::with_prefix("BUBBLE-SERVICES")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Configuration>()
}
