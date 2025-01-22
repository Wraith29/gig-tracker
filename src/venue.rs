use ratatui::widgets::Row;
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone)]
pub struct Venue {
    venue_id: i64,
    name: String,
    city_id: i64,
}

impl DataSet for Venue {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Venue, "SELECT * FROM venue")
            .fetch_all(pool)
            .await?)
    }
}

impl From<Venue> for Row<'_> {
    fn from(value: Venue) -> Self {
        Row::new(vec![
            value.venue_id.to_string(),
            value.name,
            value.city_id.to_string(),
        ])
    }
}
