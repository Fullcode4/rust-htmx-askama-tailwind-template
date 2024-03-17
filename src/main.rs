use anyhow::Context;
use api::server::Server;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod views;
pub mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let mut server = Server { assets_path, port, addr };

    let router = server.init().await;
    server.run(router).await.context("Error while starting running server...")
}

