use axum::{
    routing::{get, post},
    Json, Router,
};

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();


    return;
}

async fn root() -> &'static str {
    "Hello, World!"
}