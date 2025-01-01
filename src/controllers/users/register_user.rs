use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub success:bool,
    pub message:String
}

pub async fn register_user() -> Json<RegistrationResponse> {
    let success = true;
    let message = String::from("User successfully registeredd!!!");


    Json(RegistrationResponse {
        success,
        message
    })
}

