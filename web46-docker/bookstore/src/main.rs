use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&mut conn)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);
    Ok(())
}
