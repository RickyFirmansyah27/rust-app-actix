use actix_web::{HttpResponse, web::{Json, Path}};
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

pub async fn create_user(payload: Json<CreateUser>) -> HttpResponse {
    tracing::debug!(?payload, "Received new user payload");
    
    // Simulate user creation with a mock ID
    let user_response = UserResponse {
        id: 1,
        username: payload.username.clone(),
    };
    
    let response = BaseResponse::success(
        "User created successfully".to_string(),
        Some(user_response),
    );
    
    HttpResponse::Created().json(response)
}

pub async fn get_user(path: Path<(u32,)>) -> HttpResponse {
    let user_id = path.into_inner().0;
    tracing::debug!("Getting user with id: {}", user_id);
    
    let user_response = UserResponse {
        id: user_id,
        username: "john_doe".to_string(),
    };
    
    let response = BaseResponse::success(
        "User retrieved successfully".to_string(),
        Some(user_response),
    );
    
    HttpResponse::Ok().json(response)
}