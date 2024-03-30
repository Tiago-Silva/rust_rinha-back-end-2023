use sqlx::{Pool, MySql, Error, MySqlPool};

use dotenv::dotenv;
use std::env;

pub async fn connect() -> Result<Pool<MySql>, Error> {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL")
    //     .expect("DATABASE_URL must be set");
    return MySqlPool::connect("mysql://root:live0102@localhost:3306/pagamento").await;
}