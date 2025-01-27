use std::fmt::Display;

use ratatui::widgets::{ListItem, Row};
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone)]
pub struct City {
    pub city_id: i64,
    name: String,
}

impl DataSet for City {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(City, "SELECT * FROM city")
            .fetch_all(pool)
            .await?)
    }

    async fn save(val: Self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query("INSERT INTO \"city\" (\"name\") VALUES ($1)")
            .bind(val.name)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.clone())
    }
}

impl From<City> for Row<'_> {
    fn from(value: City) -> Self {
        Row::new(vec![value.city_id.to_string(), value.name])
    }
}

impl<'a> From<City> for ListItem<'a> {
    fn from(value: City) -> Self {
        ListItem::new(value.name.clone())
    }
}
