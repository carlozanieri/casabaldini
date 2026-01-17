// src/routes.rs
use axum::{response::Html, extract::State};
use sqlx::SqlitePool;
use crate::{models::Slider, ui::home::{Home, HomeProps}};
use dioxus_ssr::render;

/// Handler per la home page
pub async fn home(State(pool): State<SqlitePool>) -> Html<String> {
    // Estrai gli sliders dal database
    let sliders: Vec<Slider> = sqlx::query_as::<_, Slider>(
        "SELECT * FROM sliders ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default(); // se fallisce restituisce vettore vuoto

    // Render SSR con Dioxus
    let html = render(|cx| {
        // Passiamo direttamente il componente e le props
        Home(cx, HomeProps { sliders: sliders.clone() })
    });

    Html(html)
}
