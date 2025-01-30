use sqlx::{Pool, Sqlite};

use crate::error::Error;

pub trait DataSet: Sized + Clone + ToString {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error>;
    async fn save(val: Self, pool: &Pool<Sqlite>) -> Result<(), Error>;
    fn contains(&self, val: String) -> bool;
    fn key(&self) -> impl Ord + Clone;
}
