use ratatui::widgets::Row;
use sqlx::SqlitePool;

use crate::dataset::DataSet;

pub const VENUE_HEADERS: [&str; 3] = ["Venue Id", "Name", "City"];

#[derive(Clone)]
pub struct Venue {
    venue_id: i64,
    name: String,
    city: String,
}

impl<'a> From<Venue> for Row<'a> {
    fn from(value: Venue) -> Self {
        Row::new(vec![value.venue_id.to_string(), value.name, value.city])
    }
}

impl DataSet for Venue {
    async fn create(value: Self, pool: &SqlitePool) -> Result<(), crate::error::GTError> {
        todo!()
    }

    async fn load_all(pool: &SqlitePool) -> Result<Vec<Self>, crate::error::GTError> {
        Ok(sqlx::query_as!(Venue, "SELECT * FROM venue")
            .fetch_all(pool)
            .await?)
    }
}
