use axum::{
    routing::{get},
    Router,
};

//  #[path="../app/mod.rs"]
// mod app;
use crate::app::controllers::*;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(foo_controller::root))
         .route("/foo", get(foo_controller::get_foo))
}
