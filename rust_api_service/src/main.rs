mod config;
mod db;
mod errors;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let config = config::load_config().expect("Failed to load configuration");
    let db_pool = db::init_db_pool(&config).await.expect("Failed to initialize database pool");
    
    let host = config.server.host.clone();
    let port = config.server.port;
    
    info!("Starting server at {}:{}", host, port);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
