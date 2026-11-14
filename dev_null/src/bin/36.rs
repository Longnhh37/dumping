use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // libpq-style connection string
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=axum_user password=secret dbname=axum_chat_db",
        NoTls,
    )
    .await?;

    // connection object drives the socket I/O, so it must run in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    // parameterized query: $1 is bound safely (no SQL injection)
    let rows = client
        .query("SELECT id, name FROM users where id > $1", &[&0_i32])
        .await?;

    for row in rows {
        // Column types must match: Postgres INT4 -> i32, TEXT -> String
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("{id}: {name}");
    }

    Ok(())
}
