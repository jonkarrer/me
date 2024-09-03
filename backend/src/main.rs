mod router;
use std::sync::Arc;

use router::router;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let db_url = secrets
        .get("TURSO_DATABASE_URL")
        .expect("Missing TURSO_DATABASE_URL");
    let db_token = secrets
        .get("TURSO_AUTH_TOKEN")
        .expect("Missing TURSO_AUTH_TOKEN");

    let db = libsql::Builder::new_remote(db_url, db_token)
        .build()
        .await
        .expect("Failed to create db client");
    let client = Arc::new(db);

    let mut router = router().with_state(client);

    if cfg!(debug_assertions) {
        router = router.layer(tower_livereload::LiveReloadLayer::new());
    }

    Ok(router.into())
}
