use actix_web::{
    App, HttpServer, middleware, web
};
use actix_cors::Cors;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use entity::data::app_state::AppState;
use entity::data::db;

mod controller;
use controller::movie as MovieController;


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
    cfg.service(MovieController::index);
}