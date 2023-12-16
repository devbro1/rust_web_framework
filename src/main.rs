use axum::{
    routing::{get, post},
    Json, Router,
};

use lambda_http::{http::StatusCode, run, Error};


#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let routes = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
        // // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    
    if 1 == 1 // if it is aws lambda
    {
        
    
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, routes).await.unwrap();
    }

    run(routes).await
}

async fn root() -> &'static str {
    "Hello, World!"
}