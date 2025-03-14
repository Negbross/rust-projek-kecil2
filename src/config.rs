use std::env;
use std::fmt::{Display, Formatter};
use config::{Config, File, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tracing {
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configurations {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub service: Service,
    pub tracing: Tracing,
}

impl Configurations {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("___"));

        // allow to override setting environment
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        if let Ok(log_level) = env::var("LOG_LEVEL") {
            builder = builder.set_override("logger.level", log_level)?;
        }

        // Deserialize entire conf
        builder.build()?.try_deserialize()
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "http://localhost:{}", self.port)
    }
}