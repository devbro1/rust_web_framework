use axum::{
    routing::{get},
    Router,
};

pub fn get_routes() -> Router {
    let rc = Router::new()
        .route("/", get(root));
    
    return rc;
}

async fn root() -> &'static str {
    "hello world!"
}