#[cfg(test)]
mod tests {  
    use web_framework::routes::api::get_routes;
    use ::axum_test::TestServer;

    #[tokio::test]
    async fn test_add() {
        let routes = get_routes();
        let server = TestServer::new(routes).unwrap();

        let response = server.get("/").await;
        assert_eq!(response.text(),"Hello World!");
    }
}