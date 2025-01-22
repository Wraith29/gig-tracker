use ratatui::widgets::Row;
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone)]
pub struct Artist {
    artist_id: i64,
    name: String,
    city_id: i64,
}

impl DataSet for Artist {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Artist, "SELECT * FROM artist")
            .fetch_all(pool)
            .await?)
    }
}

impl From<Artist> for Row<'_> {
    fn from(value: Artist) -> Self {
        Row::new(vec![
            value.artist_id.to_string(),
            value.name,
            value.city_id.to_string(),
        ])
    }
}
