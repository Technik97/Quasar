use sea_orm::entity::prelude::*;
use sea_orm::Database;
use tokio::sync::OnceCell;
use std::env;

pub fn get_server_url() -> String {
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    server_url
} 

async fn init_conn() -> DatabaseConnection {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    
    Database::connect(&db_url)
        .await
        .expect("failed to connect to database")
}

static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_conn() -> &'static DatabaseConnection {
    CONN.get_or_init(init_conn).await
}