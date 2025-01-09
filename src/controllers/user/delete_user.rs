use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct DeleteResponse {
    pub success:bool,
    pub message:String
}

pub async fn delete_user() -> Json<DeleteResponse> {
    let success = true;
    let message = String::from("User successfully deleted!!!");


    Json(DeleteResponse {
        success,
        message
    })
}

