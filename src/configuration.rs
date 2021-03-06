use std::sync::Arc;
use serde_aux::field_attributes::deserialize_number_from_string;
use cdrs_tokio::authenticators::NoneAuthenticator;
use cdrs_tokio::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs_tokio::cluster::session::{new as new_session, Session};
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::retry::DefaultRetryPolicy;
use std::convert::{TryFrom, TryInto};

type CurrentSession = Session<RoundRobin<TcpConnectionPool>>;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: CassandraSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct CassandraSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub keyspace_name: String,
}

impl CassandraSettings {
    pub async fn get_cassandra_session(&self) -> Arc<CurrentSession> {
        let node =
            NodeTcpConfigBuilder::new(
                "127.0.0.1:9042", Arc::new(
                    NoneAuthenticator),
            ).build();
        let cluster_config = ClusterTcpConfig(vec![node]);
        let lb = RoundRobin::new();
        let session = Arc::new(
            new_session(&cluster_config, lb, Box::new(DefaultRetryPolicy::default()))
                .await
                .expect("session should be created"),
        );
        session
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Read the "default" configuration file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Layer on the environment-specific values.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}