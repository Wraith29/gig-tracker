use sqlx::SqlitePool;

use crate::error::GTError;

pub trait DataSet: Sized {
    async fn create(value: Self, pool: &SqlitePool) -> Result<(), GTError>;
    async fn load_all(pool: &SqlitePool) -> Result<Vec<Self>, GTError>;
}
