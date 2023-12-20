mod routes;
mod app;

use lambda_http::{
    // run,
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let routes = routes::api::get_routes();

    //run(routes).await
    println!("BROKEN, until lambda_http is updated to use axum:0.7.2+");

    Ok(())
}