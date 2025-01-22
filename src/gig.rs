use ratatui::widgets::Row;
use sqlx::{Pool, Sqlite};

use crate::{act::Act, dataset::DataSet, date::Date, error::Error};

#[derive(Clone)]
pub struct Gig {
    artist_id: i64,
    venue_id: i64,
    pub date: Date,
    act: Act,
}

impl DataSet for Gig {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Gig, "SELECT * FROM gig")
            .fetch_all(pool)
            .await?)
    }
}

impl From<Gig> for Row<'_> {
    fn from(value: Gig) -> Self {
        Row::new(vec![
            value.artist_id.to_string(),
            value.venue_id.to_string(),
            value.date.to_string(),
            value.act.to_string(),
        ])
    }
}
