use std::sync::Arc;
use cdrs_tokio::authenticators::NoneAuthenticator;
use cdrs_tokio::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs_tokio::cluster::session::{new as new_session, Session};
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::retry::DefaultRetryPolicy;
use std::convert::TryFrom;

type CurrentSession = Session<RoundRobin<TcpConnectionPool>>;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: CassandraSettings,
    pub application_port: u16,
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
    println!(" Has this been Called ");
    let mut settings = config::Config::default();
    println!("What is the Value {:?}", config::File::with_name("configuration.rs"));

    settings.merge(config::File::with_name("configuration"))?;



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