use ratatui::widgets::Row;
use sqlx::SqlitePool;

use crate::{dataset::DataSet, date::Date};

#[derive(Clone)]
pub enum Act {
    MainAct = 0,
    SupportAct = 1,
    SharedHeadliner = 2,
}

impl From<i64> for Act {
    fn from(value: i64) -> Self {
        match value {
            0 => Act::MainAct,
            1 => Act::SupportAct,
            2 => Act::SharedHeadliner,
            _ => Act::MainAct,
        }
    }
}

impl ToString for Act {
    fn to_string(&self) -> String {
        match self {
            Act::MainAct => String::from("Main Act"),
            Act::SupportAct => String::from("Support Act"),
            Act::SharedHeadliner => String::from("Shared Headliner"),
        }
    }
}

pub const GIG_HEADERS: [&str; 4] = ["Artist Id", "Venue Id", "Date", "Act"];

#[derive(Clone)]
pub struct Gig {
    artist_id: i64,
    venue_id: i64,
    date: Date,
    act: Act,
}

impl<'a> From<Gig> for Row<'a> {
    fn from(value: Gig) -> Self {
        Row::new(vec![
            value.artist_id.to_string(),
            value.venue_id.to_string(),
            value.date.to_string(),
            value.act.to_string(),
        ])
    }
}

impl DataSet for Gig {
    async fn create(value: Self, pool: &SqlitePool) -> Result<(), crate::error::GTError> {
        todo!()
    }

    async fn load_all(pool: &SqlitePool) -> Result<Vec<Self>, crate::error::GTError> {
        Ok(sqlx::query_as!(Gig, "SELECT * FROM gig")
            .fetch_all(pool)
            .await?)
    }
}
