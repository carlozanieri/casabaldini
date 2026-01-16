use axum::{
    extract::State,
    response::Html,
};
use sqlx::SqlitePool;
use dioxus::prelude::*;

use crate::{models::Slider, ui::home::{Home, HomeProps}};

pub async fn home(
    State(pool): State<SqlitePool>,
) -> Html<String> {
    let sliders = sqlx::query_as::<_, Slider>(
        "SELECT * FROM sliders ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    // 🔹 Dioxus 0.7 SSR corretto: passare props come struct
    let mut vdom = dioxus::prelude::VirtualDom::new_with_props(Home, HomeProps { sliders });

    // 🔹 Render SSR: Dioxus 0.7 usa render_vdom in crate dioxus_ssr
    let html = dioxus_ssr::render(&mut vdom);

    Html(html)
}
