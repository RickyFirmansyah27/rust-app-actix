use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use std::env;

pub async fn forward_to_open_router(body: web::Json<Value>) -> impl Responder {
    let client = reqwest::Client::new();
    let open_router_url = "https://openrouter.ai/api/v1/chat/completions";

    // Get the API key from an environment variable
    let api_key = match env::var("OPENROUTER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return HttpResponse::InternalServerError().json("OPENROUTER_API_KEY not set");
        }
    };

    let res = client
        .post(open_router_url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body_bytes = response.bytes().await;
            match body_bytes {
                Ok(bytes) => {
                    let mut response_builder = HttpResponse::build(status);
                    response_builder.body(bytes)
                },
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
