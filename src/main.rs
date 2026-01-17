// src/main.rs
mod db;
mod models;
mod routes;
mod ui;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Inizializza il pool SQLite
    let pool = db::init_db().await;

    // Configura il router Axum
    let app = Router::new()
        .route("/", get(routes::home))
        .with_state(pool)
        .nest_service("/static", ServeDir::new("static"));

    // Listener TCP su 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Impossibile bindare su 127.0.0.1:8080");

    println!("🚀 Server avviato su http://127.0.0.1:8080");

    // Avvia il server
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
