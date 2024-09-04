use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use libsql::params;
use walkdir::WalkDir;

use crate::{
    models::JournalEntry,
    templates::{HtmlTemplate, IndexTemplate, JournalEntriesPage},
};

fn parse_markdown_to_html(text: &str) -> String {
    let parser = pulldown_cmark::Parser::new(&text);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

pub async fn all_journal_entries(
    State(client): State<Arc<libsql::Database>>,
) -> HtmlTemplate<JournalEntriesPage> {
    let db = client.connect().expect("Failed to connect to database");

    let mut rows = db
        .query(
            "SELECT id, title, summary, content FROM journal_entries",
            (),
        )
        .await
        .expect("Failed to query database for entries");

    let mut entries = Vec::new();
    while let Some(row) = rows.next().await.expect("Failed to get next row") {
        let content: String = row.get(3).expect("Failed to get content");
        entries.push(JournalEntry {
            id: row.get(0).expect("Failed to get id"),
            title: row.get(1).expect("Failed to get title"),
            summary: row.get(2).expect("Failed to get summary"),
            content: parse_markdown_to_html(content.as_str()),
        })
    }

    HtmlTemplate(JournalEntriesPage { entries })
}

pub async fn single_journal_entry(
    Path(id): Path<u32>,
    State(client): State<Arc<libsql::Database>>,
) -> impl IntoResponse {
    let db = client.connect().expect("Failed to connect to database");

    let mut row = db
        .query(
            "SELECT title, content FROM journal_entries WHERE id = ?1",
            params!(id),
        )
        .await
        .expect("Failed to query database for single entry");

    if let Some(row) = row.next().await.expect("Failed to get next row") {
        let content: String = row.get(1).expect("Failed to get content");

        let html =r#"
    <html>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css">
        <link rel="stylesheet" href="/styles/output.css">
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <body>"#.to_string() + &parse_markdown_to_html(&content) + "<script>hljs.highlightAll();</script></body>
    </html>";

        return Ok(Html(html));
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

pub async fn index_page() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate)
}
