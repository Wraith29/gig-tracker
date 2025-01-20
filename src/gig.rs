use ratatui::widgets::Row;
use sqlx::SqlitePool;

use crate::date::Date;

#[derive(Clone)]
enum Act {
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

impl Gig {}

impl<'a> Into<Row<'a>> for Gig {
    fn into(self) -> Row<'a> {
        Row::new(vec![
            self.artist_id.to_string(),
            self.venue_id.to_string(),
            self.date.to_string(),
            self.act.to_string(),
        ])
    }
}

pub async fn get_all_gigs(pool: &SqlitePool) -> Result<Vec<Gig>, sqlx::Error> {
    Ok(sqlx::query_as!(Gig, "SELECT * FROM gig")
        .fetch_all(pool)
        .await?)
}
