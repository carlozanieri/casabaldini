use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Slider {
    pub id: i64,
    pub img: String,
    pub titolo: String,
    pub caption: String,
    pub testo: String,
}