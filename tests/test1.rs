#[cfg(test)]
mod tests {

    // use axum::{
    //     body::Body,
    //     extract::connect_info::MockConnectInfo,
    //     http::{self, Request, StatusCode},
    // };

    // mod app;
    // mod routes;

    // use web_framework::routes;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn hello_world() {
        // let _app = routes::api::get_routes();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        // let response = app
        //     .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        //     .await
        //     .unwrap();

        // assert_eq!(response.status(), StatusCode::OK);

        // let body = response.into_body().collect().await.unwrap().to_bytes();
        // assert_eq!(&body[..], b"Hello, World!");
    }
}