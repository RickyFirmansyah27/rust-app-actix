use rust_app::routes::configure_routes;
use rust_app::helpers::logger::init_logger;
use actix_web::{App, HttpServer, middleware::Logger};
use dotenvy::dotenv;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables from .env file
    dotenv().ok();

    // initialize tracing
    init_logger();

    // run our app with actix web
    tracing::debug!("listening on 0.0.0.0:8080");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(configure_routes)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}