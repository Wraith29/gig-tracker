use ratatui::widgets::Row;
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

#[derive(Clone)]
pub struct City {
    city_id: i64,
    name: String,
}

impl DataSet for City {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(City, "SELECT * FROM city")
            .fetch_all(pool)
            .await?)
    }
}

impl<'a> From<City> for Row<'a> {
    fn from(value: City) -> Self {
        Row::new(vec![value.city_id.to_string(), value.name])
    }
}
