use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use libsql::params;

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
            "SELECT id, title, summary, content FROM journal_entries ORDER BY id DESC",
            (),
        )
        .await
        .expect("Failed to query database for entries");

    let mut entries = Vec::new();
    while let Some(row) = rows.next().await.expect("Failed to get next row") {
        let content: String = row.get(3).expect("Failed to get content");
        entries.push(JournalEntry {
            id: row.get(0).expect("Failed to get id"),
            title: row
                .get::<String>(1)
                .expect("Failed to get title")
                .split_whitespace()
                .map(|s| {
                    let mut chars = s.chars();
                    let first = chars.next().unwrap().to_uppercase();
                    let rest = chars.collect::<String>();
                    format!("{}{}", first, rest)
                })
                .collect::<Vec<String>>()
                .join(" "),
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
        let title: String = row.get(0).expect("Failed to get title");
        let content: String = row.get(1).expect("Failed to get content");

        let html = format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link href="https://cdn.jsdelivr.net/npm/prismjs@v1.x/themes/prism-tomorrow.css" rel="stylesheet" />
            <script src="https://cdn.jsdelivr.net/npm/prismjs@v1.x/components/prism-core.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/prismjs@v1.x/plugins/autoloader/prism-autoloader.min.js"></script>
            <link rel="stylesheet" href="/styles/journal_entries.css">
            <title>{}</title>
        </head>
        <body>
        <main>
        <header>
            <a href="/">Jon Karrer</a>
            <nav>
                <a href="/journal">Journal</a>
                <a href="/#projects">Projects</a>
            </nav>
        </header>
        
        <section>
        {}
        </section>
        <footer>
            <p>© 2024 Jon Karrer</p>
            <p>Built with Rust 🦀</p>
        </footer>
        </main>
        <script>Prism.highlightAll();</script> 
        </body>
        </html>
        "#,
            title,
            &parse_markdown_to_html(&content)
        );

        return Ok(Html(html));
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

pub async fn index_page() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate)
}
