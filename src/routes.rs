use axum::response::Html;
use dioxus_ssr::render_to_string;
use sqlx::SqlitePool;

use crate::{models::Slider, ui::home::Home};

pub async fn home(pool: SqlitePool) -> Html<String> {
    let sliders = sqlx::query_as::<_, Slider>(
        "SELECT * FROM sliders ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let html = render_to_string(|| rsx! {
        Home { sliders }
    });

    Html(html)
}