use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Result, Responder, middleware, web, get
};
use actix_cors::Cors;
use entity::{movie, production ,sea_orm::EntityTrait};
use entity::movie::Entity as Movie;
use entity::production::Entity as Production;
use entity::sea_orm;
use serde_json::to_string;
use listenfd::ListenFd;
use serde::Serialize;
use std::env;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}


#[get("/")]
async fn index(
    data: web::Data<AppState>
) -> HttpResponse {
    let conn = &data.conn;

    let movies: Vec<(movie::Model, Option<production::Model>)> = Movie::find()
                                        .find_also_related(Production)
                                        .all(conn)
                                        .await
                                        .unwrap();

    let body = to_string(&movies).unwrap();
    
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
    
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let mut listenfd = ListenFd::from_env();
    
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // let client_url = env::var("CLIENT_URL").expect("CLIENT_URL is not set in .env file");

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let mut server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None =>  server.bind(&server_url)?,
    };

    println!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}