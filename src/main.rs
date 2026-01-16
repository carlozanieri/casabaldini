mod db;
mod models;
mod routes;
mod ui;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    let app = Router::new()
        .route("/", get(routes::home))
        .with_state(pool)
        .nest_service("/static", ServeDir::new("static"));

    let listener = TcpListener::bind("0.0.0.0:8888")
        .await
        .unwrap();

    println!("🚀 Server avviato su http://0.0.0.0:8888");

    axum::serve(listener, app)
        .await
        .unwrap();
}
