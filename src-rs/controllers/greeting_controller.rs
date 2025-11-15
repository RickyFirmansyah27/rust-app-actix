use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use crate::helpers::base_response::BaseResponse;

pub async fn greeting() -> impl IntoResponse {
    let response = BaseResponse::success(
        "Welcome to Rust Service".to_string(),
        None::<serde_json::Value>,
    );
    
    (StatusCode::OK, Json(response))
}