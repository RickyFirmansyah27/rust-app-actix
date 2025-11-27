use actix_web::HttpResponse;
use crate::helpers::base_response::BaseResponse;

pub async fn hello() -> HttpResponse {
    let response = BaseResponse::success(
        "Hello from Rust Service".to_string(),
        None::<serde_json::Value>,
    );
    
    HttpResponse::Ok().json(response)
}