use axum::Json;
use serde_json::{Value, json};

pub async fn root() -> &'static str {
    "Hello World!"
}

pub async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}
