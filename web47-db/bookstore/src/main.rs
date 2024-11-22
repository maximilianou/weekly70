use std::error::Error;
use sqlx::Row;
use sqlx::FromRow;


#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub author_id: i64,
}
#[derive(Debug, FromRow)]
struct Author {
    pub name: String,
    pub author_id: i64,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {    
//    Ok( first_steps().await? )
}
