use axum::{
    routing::{get},
    Router,
};

use std::net::SocketAddr;

mod routes;
use routes::get_routes;

mod routes {
    pub mod api;
}

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



