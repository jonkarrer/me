mod router;
use axum::{routing::get, Router};
use router::*;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(router::all_journal_entries));

    Ok(router.into())
}
