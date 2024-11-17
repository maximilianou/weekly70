use std::error::Error;
use sqlx::Row;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn updating(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = %1, author  = %2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let row = query.fetch_one(conn).await?;
//    let row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    };
    Ok(book)
}

async fn read2(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let maybe_row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = maybe_row.map( |row| {Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    }});
    Ok(book)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);


    let book = Book{
        title:  "Men searching for meaning".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };
    create(&book, &pool).await?;

    let bookUpdate = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };


    updating(&bookUpdate, &bookUpdate.isbn, &pool).await?;

    Ok(())
}
