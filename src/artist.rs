use std::fmt::Display;

use ratatui::widgets::{ListItem, Row};
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Artist {
    pub artist_id: i64,
    pub name: String,
    pub city_id: i64,
    city_name: Option<String>,
}

impl Ord for Artist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Artist {
    pub fn new(name: String, city_id: i64) -> Self {
        Self {
            artist_id: 0,
            name,
            city_id,
            city_name: None,
        }
    }
}

impl DataSet for Artist {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Artist, "SELECT 'a'.'artist_id', 'a'.'name', 'a'.'city_id', 'c'.'name' AS 'city_name' FROM 'artist' a INNER JOIN 'city' c ON 'c'.'city_id' = 'a'.'city_id'")
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

    fn contains(&self, val: String) -> bool {
        let search = val.as_str().to_lowercase();

        self.name.to_lowercase().contains(&search)
            || self
                .city_name
                .as_ref()
                .is_some_and(|name| name.to_lowercase().contains(&search))
    }

    fn key(&self) -> impl Ord + Clone {
        &self.name
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
            value.city_name.unwrap(),
        ])
    }
}

impl From<Artist> for ListItem<'_> {
    fn from(value: Artist) -> Self {
        ListItem::new(value.name.clone())
    }
}
