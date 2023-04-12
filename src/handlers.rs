use crate::models::{CreateUserRequest, ErrorMessage, User};
use crate::repository::UserRepository;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    result::Error as DieselError,
    PgConnection,
};
use log::{error, info};

fn internal_error(e: diesel::result::Error) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorMessage {
        reason: format!("{}", e),
    })
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    payload: web::Json<CreateUserRequest>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::create_user(&mut conn, &payload.username, &payload.email) {
        Ok(user) => {
            info!("Inserted: {:?}", user);
            HttpResponse::Created().json(user)
        }
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            internal_error(e)
        }
    }
}

#[get("/users/{id}")]
pub async fn get_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    id: web::Path<String>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::get_user(&mut conn, id.into_inner()) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to get user: {:?}", e);
            internal_error(e)
        }
    }
}

#[get("/users")]
pub async fn get_users(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::count_users(&mut conn) {
        Ok(number) => HttpResponse::Ok().json(number),
        Err(e) => {
            error!("Failed to fetch users: {:?}", e);
            internal_error(e)
        }
    }
}

#[patch("/users/{id}")]
pub async fn update_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    id: web::Path<String>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");

    match UserRepository::get_user(&mut conn, id.into_inner()) {
        Ok(Some(fetched)) => {
            let updated_user = User {
                id: fetched.id,
                username: user.username.clone(),
                email: user.email.clone(),
            };
            match UserRepository::update_user(&mut conn, &updated_user) {
                Ok(user) => {
                    info!("Updated: {:?}", user);
                    HttpResponse::Ok().json(user)
                }
                Err(DieselError::NotFound) => HttpResponse::NotFound().finish(),
                Err(e) => {
                    error!("Failed to update user: {:?}", e);
                    internal_error(e)
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to get user: {:?}", e);
            internal_error(e)
        }
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    id: web::Path<String>,
) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get a connection from the pool");
    let id = id.into_inner();
    match UserRepository::delete_user(&mut conn, id.clone()) {
        Ok(1) => {
            info!("Deleted user: {}", id);
            HttpResponse::NoContent().finish()
        }
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to delete user: {:?}", e);
            internal_error(e)
        }
    }
}

#[actix_rt::test]
async fn create_user_test() {
    use crate::create_connection_pool;
    use actix_web::{http::StatusCode, test, web, App};
    use dotenv::dotenv;
    use serde_json::json;

    dotenv().ok();
    env_logger::init();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(create_connection_pool().clone()))
            .service(create_user),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&json!({ "username": "test_user", "email": "test@example.com" }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
}
