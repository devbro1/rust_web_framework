mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::api::get_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening for incoming requests");
    axum::serve(listener, routes).await.unwrap();


    return;
}

// #[tokio::main]
// async fn main() {
//     let routes = ::std::net::routes::api::get_routes();

//     let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
//     println!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(routes.into_make_service())
//         .await
//         .unwrap();
// }