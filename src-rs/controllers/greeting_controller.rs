use actix_web::HttpResponse;
use crate::helpers::base_response::BaseResponse;
use tracing;

#[tracing::instrument]
pub async fn greeting() -> HttpResponse {
    let response = BaseResponse::success(
        "Welcome to Rust Service".to_string(),
        None::<serde_json::Value>,
    );
    
    HttpResponse::Ok().json(response)
}