use std::fmt::Display;

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

    async fn save(val: Self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query("INSERT INTO \"gig\" (\"artist_id\", \"venue_id\", \"date\", \"act\") VALUES ($1, $2, $3, $4)")
            .bind(val.artist_id)
            .bind(val.venue_id)
            .bind(val.date.to_string())
            .bind(val.act.to_string())
            .execute(pool).await?;

        Ok(())
    }
}

impl Display for Gig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.date, self.act)
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
