use dotenv::dotenv;
use sqlx::{FromRow, PgPool};
use std::env;
use std::error::Error;

#[derive(Debug, FromRow)]
struct Book {
    isbn: String,
    title: String,
    author: String,
}

async fn create_book(book: &Book, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (isbn, title, author) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.isbn)
        .bind(&book.title)
        .bind(&book.author)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update_book(book: &Book, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1 WHERE isbn = $2";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read_book(isbn: &str, pool: &PgPool) -> Result<Book, Box<dyn Error>> {
    let query = "SELECT * FROM book WHERE isbn = $1";

    let book = sqlx::query_as::<_, Book>(query)
        .bind(isbn)
        .fetch_one(pool)
        .await?;

    Ok(book)
}

async fn transaction_book(book: Book, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let mut txn = pool.begin().await?;

    let author_q = r"INSERT INTO author(name) VALUES ($1) RETURNING id";

    let book_q = r"INSERT INTO book(title, author, isbn) VALUES ($1, $2, $3)";

    let author_id: (i64,) = sqlx::query_as(author_q)
        .bind(&book.author)
        .fetch_one(&mut *txn)
        .await?;

    sqlx::query(book_q)
        .bind(&book.title)
        .bind(&author_id.0)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    txn.commit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let key = "CONNECTION_STRING";

    let conn_string = match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    };

    let pool = sqlx::postgres::PgPool::connect(conn_string.as_str()).await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;

    let mut book = Book {
        title: "Salem's Lot".to_string(),
        isbn: "978-0-385-00751-1".to_string(),
        author: "Stephen King".to_string(),
    };

    create_book(&book, &pool).await?;

    book.title = "The Stand".to_string();

    update_book(&book, &pool).await?;

    let book = read_book(&book.isbn, &pool).await?;

    println!("success : {book:?}");

    let book = Book {
        title: "Midnight".to_string(),
        isbn: "978-0-385-00751-2".to_string(),
        author: "Stephen King".to_string(),
    };

    transaction_book(book, &pool).await?;

    Ok(())
}
