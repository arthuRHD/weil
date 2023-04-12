extern crate diesel;
extern crate dotenv;

use crate::config::*;
use crate::handlers::*;
use actix_web::{web, App, HttpServer};

use dotenv::dotenv;
use std::env;

mod config;
mod handlers;
mod models;
mod repository;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let connection_pool = web::Data::new(create_connection_pool());

    HttpServer::new(move || {
        App::new()
            .app_data(connection_pool.clone())
            .app_data(create_json_config())
            .service(get_user)
            .service(get_users)
            .service(update_user)
            .service(delete_user)
            .service(create_user)
    })
    .bind(env::var("SOCKET_ADDRESS").expect("Socket address must be set"))?
    .run()
    .await
}
