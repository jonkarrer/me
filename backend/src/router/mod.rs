mod journal;
use std::sync::Arc;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
pub use journal::*;
use tower_http::services::ServeDir;

pub fn router() -> Router<Arc<libsql::Database>> {
    Router::new()
        .route("/", get(index))
        .nest_service("/styles", ServeDir::new("frontend/styles"))
        .route("/journal", get(all_journal_entries_from_db))
        .route("/journal/:id", get(get_journal_entry))
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(HelloTemplate)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
