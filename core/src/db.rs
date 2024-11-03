use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::{NoTls, Client, Error};

// Initialize the PostgreSQL connection pool
pub async fn create_pg_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.host = Some("postgres-service:5432".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("password".to_string());
    cfg.dbname = Some("stock_data".to_string());
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}

pub async fn connect_to_db() -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=postgres-service user=postgres password=password dbname=stock_data", NoTls)
        .await?;
    tokio::spawn(connection); // Run the connection in a separate task
    Ok(client)
}