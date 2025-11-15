use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use crate::helpers::base_response::BaseResponse;
use tracing;

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub username: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    tracing::debug!(?payload, "Received new user payload");
    
    // Simulate user creation with a mock ID
    let user_response = UserResponse {
        id: 1,
        username: payload.username,
    };
    
    let response = BaseResponse::success(
        "User created successfully".to_string(),
        Some(user_response),
    );
    
    (StatusCode::CREATED, Json(response))
}

pub async fn get_user() -> impl IntoResponse {
    let user_response = UserResponse {
        id: 1,
        username: "john_doe".to_string(),
    };
    
    let response = BaseResponse::success(
        "User retrieved successfully".to_string(),
        Some(user_response),
    );
    
    (StatusCode::OK, Json(response))
}