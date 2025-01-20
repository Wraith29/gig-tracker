use ratatui::{layout::Constraint, widgets::Row};
use sqlx::SqlitePool;

use crate::{dataset::DataSet, error::GTError};

pub const ARTIST_HEADERS: [&str; 3] = ["Artist Id", "Name", "From"];

#[derive(Clone)]
pub struct Artist {
    artist_id: i64,
    name: String,
    from: String,
}

impl<'a> Into<Row<'a>> for Artist {
    fn into(self) -> Row<'a> {
        Row::new(vec![self.artist_id.to_string(), self.name, self.from])
    }
}

pub async fn get_all_artists(pool: &SqlitePool) -> Result<Vec<Artist>, sqlx::Error> {
    Ok(sqlx::query_as!(Artist, "SELECT * FROM artist")
        .fetch_all(pool)
        .await?)
}

impl DataSet for Artist {
    async fn load_all(pool: &SqlitePool) -> Result<Vec<Artist>, GTError> {
        Ok(sqlx::query_as!(Artist, "SELECT * FROM artist")
            .fetch_all(pool)
            .await?)
    }

    async fn create(value: Self, pool: &SqlitePool) -> Result<(), GTError> {
        sqlx::query!(
            "INSERT INTO artist (\"name\", \"from\") VALUES (?1, ?2)",
            value.name,
            value.from
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
