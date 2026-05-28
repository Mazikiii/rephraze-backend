use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct RedisSettings {}

#[derive(serde::Deserialize)]
pub struct DeepgramSettings {}

#[derive(serde::Deserialize)]
pub struct CloudflareSettings {}

#[derive(serde::Deserialize)]
pub struct FirebaseSettings {}

#[derive(serde::Deserialize)]
pub struct GeminiSettings {}

#[derive(serde::Deserialize)]
pub struct ExaSettings {}

pub fn get_configurations() -> Result<Settings, config::ConfigError> {
    // initialize the config reader then immediately add the source you're reading from and lastly build
    let mut settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;

    settings.try_deserialize()
}
