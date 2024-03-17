use axum::response::IntoResponse;

use super::HtmlTemplate;

use super::templates::{IndexTemplate, DashboardTemplate};


pub async fn index() -> impl IntoResponse {
    let index = IndexTemplate {};
    HtmlTemplate(index)
}

pub async fn dashboard() -> impl IntoResponse {
    let dashboard = DashboardTemplate {};
    HtmlTemplate(dashboard)
}