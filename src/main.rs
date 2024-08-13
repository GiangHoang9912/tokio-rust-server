use axum::{routing::get, Router};
use setup::setup;

fn generate_api(routes: Vec<String>) {
  for route in routes {
    println!("api: {}", route);
  }
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let host = "0.0.0.0";
  let port = 3000;
  let pre_fix = "/api/v1";
  let num_cpus = num_cpus::get();

  let app = Router::new().route(&format!("{pre_fix}/health"), get(root));

  let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await.unwrap();

  let routes =
    [format!("http://{host}:{port}, method: GET"), format!("http://{host}:{port}{pre_fix}/health, method: GET")];

  generate_api(routes.to_vec());

  setup!(port, num_cpus);

  axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
  "Hello, World!"
}
