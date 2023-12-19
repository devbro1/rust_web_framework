use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{Value, json};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo))
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}