use std::fmt::Display;

use ratatui::widgets::Row;
use sqlx::{Pool, Sqlite};

use crate::{act::Act, dataset::DataSet, date::Date, error::Error};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Gig {
    artist_id: i64,
    venue_id: i64,
    pub date: Date,
    act: Act,

    artist_name: Option<String>,
    venue_name: Option<String>,
}

impl Ord for Gig {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl Gig {
    pub fn new(artist_id: i64, venue_id: i64, date: Date, act: Act) -> Self {
        Self {
            artist_id,
            venue_id,
            date,
            act,
            artist_name: None,
            venue_name: None,
        }
    }
}

impl DataSet for Gig {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Gig,
            r#"
            SELECT 'g'.'artist_id', 'g'.'venue_id', 'g'.'date', 'g'.'act',
                   'a'.'name' AS 'artist_name', 'v'.'name' AS 'venue_name'
            FROM 'gig' g
            INNER JOIN 'artist' a ON 'a'.'artist_id' = 'g'.'artist_id'
            INNER JOIN 'venue' v ON 'v'.'venue_id' = 'g'.'venue_id'"#
        )
        .fetch_all(pool)
        .await?)
    }

    async fn save(val: Self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query("INSERT INTO \"gig\" (\"artist_id\", \"venue_id\", \"date\", \"act\") VALUES ($1, $2, $3, $4)")
            .bind(val.artist_id)
            .bind(val.venue_id)
            .bind(val.date.to_string())
            .bind(val.act as i64)
            .execute(pool).await?;

        Ok(())
    }

    fn contains(&self, val: String) -> bool {
        let search = val.as_str().to_lowercase();

        self.artist_name
            .as_ref()
            .is_some_and(|name| name.to_lowercase().contains(&search))
            || self
                .venue_name
                .as_ref()
                .is_some_and(|name| name.to_lowercase().contains(&search))
    }

    fn key(&self) -> impl Ord + Clone {
        &self.date
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
            value.artist_name.unwrap(),
            value.venue_name.unwrap(),
            value.date.to_string(),
            value.act.to_string(),
        ])
    }
}
