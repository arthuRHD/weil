use actix_web::{error, web, web::JsonConfig, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;

pub fn create_json_config() -> JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _| {
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        })
}

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}
