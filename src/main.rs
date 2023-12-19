use axum::{
    routing::{get, post},
    Json, Router,
};

mod routes;

pub mod my_module;

#[tokio::main]
async fn main() {
    let routes = routes::api::get_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening for incoming requests");
    axum::serve(listener, routes).await.unwrap();


    return;
}
