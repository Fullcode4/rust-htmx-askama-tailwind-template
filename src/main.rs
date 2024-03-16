use anyhow::Context;
use askama::Template;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let router = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("error while creating listener")?;

    info!("router initialized, now listening on port {}", port);

    axum::serve(listener, router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn index() -> impl IntoResponse {}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render template. Error: {}", e)).into_response(),
        }
    }
}
