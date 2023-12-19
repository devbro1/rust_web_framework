use pretty_assertions::assert_eq;

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

#[cfg(test)]
mod tests {  

    use web_framework::routes;
    use ::axum_test::TestServer;

    #[test]
    fn test_add() {
        let routes = routes::api::get_routes();
        let server = TestServer::new(routes).unwrap();

        let response = aw!(server.get("/").await);
        println!("{}",response);


        // let a = aw!(root());

        // assert_eq!(a,"Hello, World!");
        // assert_eq!(2,2);
    }


    #[test]
    fn test_add2() {
        // let a = routes::api::get_routes();

        // assert_eq!(a,"func2");
        // assert_eq!(2,2);
    }
}