use std::process::exit;

use sqlx::postgres::PgPoolOptions;

pub async fn init_sql() -> sqlx::Pool<sqlx::Postgres> {
    // This project uses PostgreSQL
    match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://toakuai@localhost/toaq")
        .await
    {
        Err(x) => {
            println!("Error connecting to SQL Server!\n{}", x);
            exit(-1);
        }
        Ok(x) => x,
    }
}
