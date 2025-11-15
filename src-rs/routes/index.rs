use axum::{
    routing::{get, post},
    Router,
};
use crate::controllers::{
    greeting_controller::greeting,
    hello_controller::hello,
    user_controller::{create_user, get_user},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(greeting))
        .route("/hello", get(hello))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
}