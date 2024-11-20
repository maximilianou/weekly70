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


async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn, author_id) VALUES ($1, $2, $3, $4)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .bind(&book.author_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn updating(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author  = $2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn, author_id FROM book";

    let query = sqlx::query_as::<_, Book>(q);
    
//    let row = query.fetch_one(conn).await?;
//    let row = query.fetch_optional(conn).await?;
//    let book = Book{
//        title: row.get("title"),
//        author: row.get("author"),
//        isbn: row.get("isbn"),
//    };

    let books = query.fetch_all(conn).await?;
    
    Ok(books)
}

async fn read2(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn, author_id FROM book";
    let query = sqlx::query(q);
    let maybe_row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = maybe_row.map( |row| {Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
        author_id: row.get("author_id"),
    }});
    Ok(book.unwrap())
}

async fn first_steps() -> Result<(), Box<dyn Error>>{
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);

/*
    let book = Book{
        title:  "Men searching for meaning".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };
    create(&book, &pool).await?;
*/
    let book_update = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
        author_id: 0,
    };


    updating(&book_update, &book_update.isbn, &pool).await?;

    let book_read = read(&pool).await?;

//    println!("book 1: {}", &book_read.title);
    println!("{:?}", &book_read);

    let book_read2 = read2(&pool).await?;

    println!("book 2: {}", &book_read2.title);

    Ok(())

}

async fn insert_book(
    book: Book, conn: &sqlx::PgPool
  ) -> Result<(), Box<dyn Error>>{    
      let mut txn = conn.begin().await?;
      let author_q = r"
          INSERT INTO author (name) VALUES ($1) RETURNING id
      ";
      let book_q = r"
          INSERT INTO book (title, author_id, isbn) VALUES ($1, $2, $3)
      ";
      let author_id: (i64,) = sqlx::query_as::<_, (i64,)>(author_q)
          .bind(&book.author)
          .fetch_one(&mut txn)
          .await?;
      sqlx::query(book_q)
          .bind(&book.title)
          .bind(&author_id.0)
          .bind(&book.isbn)
          .execute(&mut txn)
          .await?;
      txn.commit().await?;
      Ok(())
  }
  async fn second_steps() -> Result<(), Box<dyn Error>>{    
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let book_insert = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "444-3-234-12345-2".to_string(),
        author_id: 0,
    };


    Ok( insert_book(book_insert, &pool).await? )
  }
  
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {    
//    Ok( first_steps().await? )
    Ok( second_steps().await? )
}


