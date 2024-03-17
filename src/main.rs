use anyhow::Context;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::views::routes::{dashboard, index};

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

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let router = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(dashboard))
        .nest_service(
            "/assets", 
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap()))
        );
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("error while creating listener")?;

    info!("router initialized, now listening on port {}", port);

    axum::serve(listener, router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

