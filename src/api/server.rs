use std::{net::SocketAddr, path::PathBuf};

use anyhow::Context;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::views::routes::{dashboard, index};


// TODO: Change name to be more descriptive
// TODO: Later check if this variables are needed or not
pub struct Server {
    pub assets_path: PathBuf,
    pub port: u16,
    pub addr: SocketAddr,
}

impl Server {

    pub fn new(assets_path: PathBuf, port: u16) -> Self {
        Self {
            assets_path,
            port,
            addr: SocketAddr::from(([0, 0, 0, 0], port)),
        }
    }

    pub async fn init(&mut self) -> Router {

        info!("initializing router...");

        let router = Router::new()
            .route("/", get(index))
            .route("/dashboard", get(dashboard))
            .nest_service(
                "/assets", 
                ServeDir::new(format!("{}/assets", self.assets_path.to_str().unwrap())));

        router
    }

    pub async fn run(&self, router: Router) -> anyhow::Result<()> {
        info!("starting server at port {}", self.port);        
        let listener = TcpListener::bind(self.addr).await.context("Error while binding address")?;
        axum::serve(listener, router).await.context("Error while staring server")?;
        Ok(())
    }

}