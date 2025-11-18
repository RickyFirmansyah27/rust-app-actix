use actix_web::{web, Scope};
use crate::controllers::{
    greeting_controller::greeting,
    hello_controller::hello,
    user_controller::{create_user, get_user},
};

pub fn create_routes() -> Scope {
    web::scope("")
        .route("/", web::get().to(greeting))
        .route("/hello", web::get().to(hello))
        .route("/users", web::post().to(create_user))
        .route("/users/{id}", web::get().to(get_user))
}