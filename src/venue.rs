use std::fmt::Display;

use ratatui::widgets::{ListItem, Row};
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Venue {
    pub venue_id: i64,
    name: String,
    city_id: i64,
    city_name: Option<String>,
}

impl Ord for Venue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Venue {
    pub fn new(name: String, city_id: i64) -> Self {
        Self {
            venue_id: 0,
            name,
            city_id,
            city_name: None,
        }
    }
}

impl DataSet for Venue {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Venue, "SELECT 'v'.'venue_id', 'v'.'name', 'v'.'city_id', 'c'.'name' AS 'city_name' FROM 'venue' v INNER JOIN 'city' c ON 'c'.'city_id' = 'v'.'city_id'")
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
            value.city_name.unwrap(),
        ])
    }
}

impl From<Venue> for ListItem<'_> {
    fn from(value: Venue) -> Self {
        ListItem::new(value.name.clone())
    }
}
