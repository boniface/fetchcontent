use std::sync::Arc;

use cdrs_tokio::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::retry::DefaultRetryPolicy;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(serde::Deserialize)]
pub struct CassandraSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub keyspace_name: Str,
}

impl CassandraSettings {
    pub fn getCassandraSession(&self) -> Arc<CurrentSession> {
        let node =
            NodeTcpConfigBuilder::new(
                "127.0.0.1:9042", Arc::new(
                    NoneAuthenticatorProvider)
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

    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}