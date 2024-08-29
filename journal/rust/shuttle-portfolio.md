# Shuttle Portfolio

Shuttle is an awesome framework for building and deploying all things Rust. I have been wanting to move my website over to their platform for some time, and add some new features. So here is the project.

## Getting Started

First we need to get the project up and running. Follow the [instructions here](https://docs.shuttle.rs/getting-started/installation). I chose axum for my framework.

## Journal

I want to incorporate some of my journal entries into the portfolio. I enjoy documenting my thoughts and processes while I build various projects, so this will be a good place to store those entries.

### Architecture

We will need a `journal` page that displays short summaries of the journal entries, and then have those linked to the full entry. Markdown is my preferred format for the entries, so we need a way to convert it to HTML.

- [pulldown-cmark](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/) is used for the conversion. Extremely easy to use, here is an example:

```rust
fn parse_markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}
```

And then simply serve with axum:

```rust
pub async fn serve() -> Html<String> {
    let html = parse_markdown_to_html("# My Journal").unwrap());

    // add syntax highlighting for code blocks
    Html(r#"
    <html>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css">
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <body>"#.to_string() + &html + "<script>hljs.highlightAll();</script></body>
    </html>"
    )
}
```

Now we need a way to route to these files, and a place to store them. Since I don't want to deploy the site every time I make a new entry, we need a place to store the entries and then load them on demand. 

- [turso](https://app.turso.tech) looks like it will work

