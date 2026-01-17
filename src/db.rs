use sqlx::sqlite::SqlitePool;
use std::env;

/// Inizializza il database SQLite e ritorna un pool di connessione
pub async fn init_db() -> SqlitePool {
    // Se vuoi, puoi leggere il path dal env var DATABASE_URL
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "casabaldini.sqlite".to_string());

    // Crea il pool di connessioni
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Impossibile connettersi al database SQLite");

    // Test di connessione (opzionale)
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Il database non risponde");

    pool
}
