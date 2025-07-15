mod models;
mod router;
mod routes;
mod templates;

use router::router;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = std::env::var("TURSO_DATABASE_URL").expect("Missing TURSO_DATABASE_URL");
    let db_token = std::env::var("TURSO_AUTH_TOKEN").expect("Missing TURSO_AUTH_TOKEN");
    let port = std::env::var("APP_PORT").expect("Missing APP_PORT");

    println!("Connecting to {}...", db_url);

    let db = libsql::Builder::new_remote(db_url, db_token)
        .build()
        .await
        .expect("Failed to create db client");
    let client = Arc::new(db);

    let mut router = router().with_state(client);

    if cfg!(debug_assertions) {
        router = router.layer(tower_livereload::LiveReloadLayer::new());
    }

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
