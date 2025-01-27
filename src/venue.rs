use std::fmt::Display;

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

    async fn save(val: Self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query("INSERT INTO \"venue\" (\"name\", \"city_id\") VALUES ($1, $2)")
            .bind(val.name)
            .bind(val.city_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Display for Venue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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
