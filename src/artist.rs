use std::fmt::Display;

use ratatui::widgets::{ListItem, Row};
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone)]
pub struct Artist {
    pub artist_id: i64,
    pub name: String,
    pub city_id: i64,
}

impl Artist {
    pub fn new(name: String, city_id: i64) -> Self {
        Self {
            artist_id: 0,
            name,
            city_id,
        }
    }
}

impl DataSet for Artist {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Artist, "SELECT * FROM artist")
            .fetch_all(pool)
            .await?)
    }

    async fn save(val: Self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query("INSERT INTO artist (\"name\", \"city_id\") VALUES ($1, $2)")
            .bind(val.name)
            .bind(val.city_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Display for Artist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.clone())
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

impl From<Artist> for ListItem<'_> {
    fn from(value: Artist) -> Self {
        ListItem::new(value.name.clone())
    }
}
