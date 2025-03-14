use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::prelude::*;
use crate::config::Configurations;
use crate::router::create_router;

mod model;
mod handlers;
mod request;
mod config;
mod database;
mod router;
mod shutdown;
mod schema;
mod middlewares;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{:?}=trace", env!("CARGO_CRATE_NAME")).into()))
        .init();

    dotenv().ok();
    // Load conf
    let config = Configurations::new().expect("Error load conf");

    let app_state = database::get_connection_pool(&config);
    let app = create_router(app_state);

    let rx = shutdown::register();

    let address : SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse().expect("Error parsing address");
    println!("boot the address was: {:?}", address);
    let listener = TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app.into_make_service()).with_graceful_shutdown(
        async {
            rx.await.ok();
            info!("Handling graceful shutdown");
            info!("Close resources, drain and shutdown event handler... etc");
        }
    ).await.expect("Error starting graceful shutdown");

}
