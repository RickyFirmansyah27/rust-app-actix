use actix_web::{web, HttpResponse, Responder};
use tracing;
use serde_json::Value;
use std::env;
use std::time::Duration;

const API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const REFERER: &str = "https://online-code-preview.vercel.app";
const TITLE: &str = "Online Code Preview";
const TIMEOUT_SECONDS: u64 = 300;


pub async fn forward_to_open_router(body: web::Json<Value>) -> impl Responder {
    tracing::info!("Received chat completion request");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .build()
        .unwrap();

    // Get the API key from an environment variable
    let api_key = match env::var("OPENROUTER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return HttpResponse::InternalServerError().json("OPENROUTER_API_KEY not set");
        }
    };

    let res = client
        .post(API_URL)
        .bearer_auth(api_key)
        .header("HTTP-Referer", REFERER)
        .header("X-Title", TITLE)
        .json(&body)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            tracing::info!("OpenRouter responded with status: {}", status);

            if status.is_success() {
                tracing::info!("Chat completion response forwarded successfully");
            }
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
