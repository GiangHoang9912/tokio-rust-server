use config::{Config, ConfigError};
use dotenv::var;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Postgres {
  pub host: String,
  pub port: u32,
  pub user: String,
  pub password: String,
  pub database: String,
  pub max_connection: u32,
}

#[derive(Deserialize)]
pub struct WebConfig {
  pub host: String,
  pub port: u32,
}

#[derive(Deserialize)]
pub struct ProdConfig {
  pub prod_web: WebConfig,
  pub prod_postgres: Postgres,
}

#[derive(Deserialize)]
pub struct DevConfig {
  pub dev_web: WebConfig,
  pub dev_postgres: Postgres,
}

impl ProdConfig {
  pub fn from_env() -> Result<ProdConfig, ConfigError> {
    match var("ENV").as_deref() {
      Ok("prod") => {
        let config = Config::builder()
          .add_source(config::Environment::default())
          .build()
          .expect("Can't loading env")
          .try_deserialize::<ProdConfig>()?;

        Ok(ProdConfig { prod_web: config.prod_web, prod_postgres: config.prod_postgres })
      },
      _ => {
        let config = Config::builder()
          .add_source(config::Environment::default())
          .build()
          .expect("Can't loading env")
          .try_deserialize::<DevConfig>()?;

        Ok(ProdConfig { prod_web: config.dev_web, prod_postgres: config.dev_postgres })
      },
    }
  }
}

impl Postgres {
  pub fn get_dsn(&self) -> String {
    format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.database)
  }
}
