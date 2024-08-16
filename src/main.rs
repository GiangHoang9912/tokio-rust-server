use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use server::{configs::ProdConfig, dbs::initialize_db};
// use server::{configs::{get_app_configs, get_dsn, get_max_connection},
use setup::setup;
use tracing::info;

fn generate_api(routes: Vec<String>) {
  for route in routes {
    info!("api: {}", route);
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();
  let pre_fix = "/api/v1";
  let num_cpus = num_cpus::get();

  let cfg = ProdConfig::from_env().expect("Can't not get config");

  let db_pool = initialize_db(cfg.prod_postgres.get_dsn().as_str(), cfg.prod_postgres.max_connection).await;
  info!("Database connection established.");
  let app = Router::new().route(&format!("{pre_fix}/health"), get(root)).layer(Extension(Arc::new(db_pool)));

  let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.prod_web.host, cfg.prod_web.port)).await.unwrap();

  let routes = [
    format!("http://{}:{}, method: {}", cfg.prod_web.host, cfg.prod_web.port, "GET"),
    format!("http://{}:{}{}/health, method: {}", cfg.prod_web.host, cfg.prod_web.port, pre_fix, "GET"),
  ];

  generate_api(routes.to_vec());

  setup!(cfg.prod_web.port, num_cpus, cfg.prod_web.host);

  axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
  "Hello, World!"
}
