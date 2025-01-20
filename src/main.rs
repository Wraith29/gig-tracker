mod app;
mod artist;
mod data_table;
mod dataset;
mod date;
mod error;
mod gig;
mod venue;

use app::App;
use dotenv::dotenv;
use error::GTError;
use std::env;

#[async_std::main]
async fn main() -> Result<(), GTError> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("Missing Required env var DATABASE_URL")
        .to_owned();

    let app = match App::new(&db_url).await {
        Ok(app) => app,
        Err(err) => return Err(err),
    };

    let result = app.run().await;

    ratatui::restore();

    result
}
