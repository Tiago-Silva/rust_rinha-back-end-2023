use sqlx::{MySql, Pool};
use crate::repository::db_config::connect;

pub struct Repository{
    connection: Pool<MySql>,
}

impl Repository {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let connection = connect().await?;
        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> Pool<MySql> {
        self.connection.clone()
    }
}