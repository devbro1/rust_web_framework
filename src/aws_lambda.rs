
use lambda_http::{
    run,
    Error,
};
use axum::{
    extract::Path,
    response::Json,
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let routes = routes::api::get_routes();

    run(routes).await
}