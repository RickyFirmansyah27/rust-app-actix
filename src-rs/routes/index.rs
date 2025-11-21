use actix_web::web;
use crate::controllers::{
    greeting_controller::greeting,
    hello_controller::hello,
    user_controller::{create_user, get_user},
    open_router_controller::forward_to_open_router,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(greeting))
            .route("/hello", web::get().to(hello))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/v1/chat/completions", web::post().to(forward_to_open_router))
    );
}