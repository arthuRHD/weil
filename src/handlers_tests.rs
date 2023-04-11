use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use serde_json::json;
use user_crud::handlers::*;

// You can write tests for each of your endpoints
#[actix_rt::test]
async fn test_create_user() {
    let app = App::new().service(web::resource("/users").route(web::post().to(create_user)));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&json!({ "username": "test_user", "email": "test@example.com" }))
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
}
