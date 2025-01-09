use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub user_name: String,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
}

pub async fn get_user() -> Json<UserResponse> {
    let user_name = String::from("JD123");
    let email_address = String::from("johndoe@mail.com");
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    Json(UserResponse {
        user_name,
        email_address,
        first_name,
        last_name,
    })
}
