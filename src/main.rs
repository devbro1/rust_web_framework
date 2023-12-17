use axum::{
    routing::{get},
    Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes = get_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

pub fn get_routes() -> Router {
    let rc = Router::new()
        .route("/", get(root));
    
    return rc;
}

async fn root() -> &'static str {
    "Hello, World!"
}