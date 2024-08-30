use std::{path::Path, sync::Arc};

use axum::{
    extract::State,
    http::{Result, StatusCode},
    response::{Html, IntoResponse},
};
use libsql::params;
use walkdir::WalkDir;

struct JournalEntrySummary {
    id: String,
    title: String,
    summary: String,
}

struct JournalEntry {
    title: String,
    content: String,
}

fn parse_markdown_to_html(text: &str) -> String {
    let parser = pulldown_cmark::Parser::new(&text);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

pub async fn all_journal_entries_from_db(
    State(client): State<Arc<libsql::Database>>,
) -> Html<String> {
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
        entries.push(JournalEntrySummary {
            id: row.get(0).expect("Failed to get id"),
            title: row.get(1).expect("Failed to get title"),
            summary: row.get(2).expect("Failed to get summary"),
        })
    }

    let mut html = String::new();
    for entry in entries {
        html.push_str(&format!("<h1>{}</h1>", entry.title));
        html.push_str(&format!("<p>{}</p>", entry.summary));
    }

    Html(html)
}

pub async fn get_journal_entry(State(client): State<Arc<libsql::Database>>) -> impl IntoResponse {
    let db = client.connect().expect("Failed to connect to database");

    let mut row = db
        .query(
            "SELECT title, content FROM journal_entries WHERE id = ?1",
            params!([1]),
        )
        .await
        .expect("Failed to query database for single entry");

    if let Some(row) = row.next().await.expect("Failed to get next row") {
        let mut html = String::new();
        let content: String = row.get(1).expect("Failed to get content");

        html.push_str(&parse_markdown_to_html(&content));

        return Ok(Html(html));
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

pub async fn all_journal_entries_from_dir() -> Html<String> {
    let mut entries = Vec::new();

    for entry in WalkDir::new("journal").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        let title = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("-", " ")
            .to_owned();
        let text = std::fs::read_to_string(entry.path()).expect("Failed to read file");
        entries.push((title, parse_markdown_to_html(&text)));
    }
    let mut html = String::new();
    for (title, content) in entries {
        html.push_str(&format!("<h1>{}</h1>", title));
        html.push_str(&content);
    }

    // Add syntax highlighting
    Html(r#"
    <html>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css">
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <body>"#.to_string() + &html + "<script>hljs.highlightAll();</script></body>
    </html>"
    )
}
