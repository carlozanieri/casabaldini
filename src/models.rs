use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Slider {
    pub id: i64,
    pub codice: String,
    pub codice2: String,
    pub img: String,
    pub titolo: String,
    pub caption: String,
    pub link: String,
    pub testo: String,
}
