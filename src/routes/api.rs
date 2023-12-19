use axum::{
    routing::{get},
    Router,
};

#[path="../app/mod.rs"]
mod app;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(app::controllers::foo_controller::root))
        .route("/foo", get(app::controllers::foo_controller::get_foo))
}
