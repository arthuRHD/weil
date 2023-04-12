extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

use crate::handlers::*;
use actix_web::{
    error,
    web::{self, JsonConfig},
    App, HttpResponse, HttpServer,
};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use std::env;

mod handlers;
mod models;
mod repository;
mod schema;

fn create_connection_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}

fn create_json_config() -> JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _| {
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_pool = web::Data::new(create_connection_pool());

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(connection_pool.clone())
            .app_data(create_json_config())
            .service(
                web::resource("/users/{id}")
                    .route(web::get().to(get_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user)),
            )
            .service(web::resource("/users").route(web::post().to(create_user)))
    })
    .bind(env::var("SOCKET_ADDRESS").expect("Socket address must be set"))?
    .run()
    .await
}
