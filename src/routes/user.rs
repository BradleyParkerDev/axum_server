use axum::{Router, routing::{delete,get,post,put}};
use crate::controllers::user::{delete_user,get_user,register_user,update_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/delete-user", delete(delete_user))
        .route("/get-user", get(get_user))
        .route("/register-user", post(register_user))
        .route("/update-user", put(update_user))
}
