use ratatui::widgets::Row;
use sqlx::SqlitePool;

pub const VENUE_HEADERS: [&str; 3] = ["Venue Id", "Name", "City"];

#[derive(Clone)]
pub struct Venue {
    venue_id: i64,
    name: String,
    city: String,
}

impl<'a> Into<Row<'a>> for Venue {
    fn into(self) -> Row<'a> {
        Row::new(vec![self.venue_id.to_string(), self.name, self.city])
    }
}

pub async fn get_all_venues(pool: &SqlitePool) -> Result<Vec<Venue>, sqlx::Error> {
    Ok(sqlx::query_as!(Venue, "SELECT * FROM venue")
        .fetch_all(pool)
        .await?)
}
