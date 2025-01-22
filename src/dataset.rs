use sqlx::{Pool, Sqlite};

use crate::error::Error;

pub trait DataSet: Sized {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error>;
}
