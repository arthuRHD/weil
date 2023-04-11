use crate::models::User;
use crate::repository::UserRepository;
use actix_web::{web, HttpResponse, Responder};
use diesel::{r2d2, result::Error as DieselError, PgConnection};

// Create
pub async fn create_user(
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
    username: web::Json<String>,
    email: web::Json<String>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::create_user(&mut conn, &username, &email) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Read
pub async fn get_user(
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::get_user(&mut conn, id.into_inner()) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to get user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Update
pub async fn update_user(
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
    id: web::Path<i32>,
    user: web::Json<User>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    let mut updated_user = user.into_inner();
    updated_user.id = id.into_inner();

    match UserRepository::update_user(&mut conn, &updated_user) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(DieselError::NotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Delete
pub async fn delete_user(
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::delete_user(&mut conn, id.into_inner()) {
        Ok(1) => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to delete user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}