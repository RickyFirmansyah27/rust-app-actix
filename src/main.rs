use rust_app::routes::configure_routes;
use rust_app::helpers::logger::init_logger;
use actix_web::{App, HttpServer, middleware::Logger};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables from .env file
    dotenv().ok();

    // initialize tracing
    init_logger();

    // run our app with actix web
    tracing::debug!("listening on 0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}