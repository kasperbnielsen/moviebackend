use axum::response::IntoResponse;
use mongodb::bson::{oid::ObjectId, serde_helpers::hex_string_as_object_id};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub _id: ObjectId,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub _id: String,
}

impl IntoResponse for User {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for UserDTO {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
