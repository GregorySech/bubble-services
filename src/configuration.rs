use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationConfiguration {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
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
