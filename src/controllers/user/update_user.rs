use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateResponse {
    pub success:bool,
    pub message:String
}

pub async fn update_user() -> Json<UpdateResponse> {
    let success = true;
    let message = String::from("User successfully updated!!!");


    Json(UpdateResponse {
        success,
        message
    })
}

