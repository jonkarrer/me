use crate::routes::*;
use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

pub fn router() -> Router<Arc<libsql::Database>> {
    Router::new()
        .route("/", get(index_page))
        // .nest_service("/styles", ServeDir::new("frontend/styles"))
        // .nest_service("/assets", ServeDir::new("frontend/assets"))
        .nest_service("/styles", ServeDir::new("./frontend/styles"))
        .nest_service("/assets", ServeDir::new("./frontend/assets"))
        .route("/journal", get(all_journal_entries))
        .route("/journal/{id}", get(single_journal_entry))
}
