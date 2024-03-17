use askama::Template;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};


pub mod templates;

pub struct HtmlTemplate<T>(T);

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