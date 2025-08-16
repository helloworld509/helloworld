use std::sync::Arc;

use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use env_logger;

mod api;
mod db;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let pool = db::create_pool().await;
    let pool_data = web::Data::new(pool);
    println!("service is running on http://localhost:9000");
    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .configure(routes::config_routes)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
