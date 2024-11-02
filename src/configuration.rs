#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub protocol: String,
}

pub struct DatabaseSettings {}

impl Settings {
    pub fn init() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::new(
                "configuration.yml",
                config::FileFormat::Yaml,
            ))
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}
