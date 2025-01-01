use axum::{Router, routing::get};
use crate::controllers::web::{home_page_controller};


pub fn web_routes() -> Router {
    Router::new()
        .route("/", get(home_page_controller))
}