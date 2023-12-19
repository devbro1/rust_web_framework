use axum::{
    routing::{get, post},
    Json, Router,
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> &'static str {
    "Hello World!"
}