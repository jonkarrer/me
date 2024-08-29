use std::path::Path;

use axum::response::Html;
use walkdir::WalkDir;

fn parse_markdown_to_html(file: &Path) -> String {
    let text = std::fs::read_to_string(file).expect("Failed to read file");

    let parser = pulldown_cmark::Parser::new(&text);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

pub async fn all_journal_entries() -> Html<String> {
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
        entries.push((title, parse_markdown_to_html(entry.path())));
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
