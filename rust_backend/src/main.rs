use actix_web::{
    App, HttpResponse, HttpServer, middleware, web, get
};
use actix_cors::Cors;
use entity::{movie, production ,sea_orm::EntityTrait};
use entity::movie::Entity as Movie;
use entity::production::Entity as Production;
use entity::sea_orm;
use serde_json::to_string;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

mod data;
use crate::data::db;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection
}

#[get("/")]
async fn index(
    data: web::Data<AppState>
) -> HttpResponse {
    let conn = &data.conn;
    //(movie::Model, Option<production::Model>)
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
    
    let conn = db::get_conn().await.to_owned();
    let server_url = db::get_server_url();
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