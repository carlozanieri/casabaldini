mod db;
mod models;
mod routes;
mod ui;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    let app = Router::new()
        .route("/", get({
            let pool = pool.clone();
            move || routes::home(pool)
        }))
        .nest_service("/static", ServeDir::new("static"));

    println!("Server avviato su http://0.0.0.0:8888");

    axum::Server::bind(&"0.0.0.0:8888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}