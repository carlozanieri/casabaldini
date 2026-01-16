use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn init_db() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:casabaldini.sqlite")
        .await
        .expect("Errore connessione SQLite")
}