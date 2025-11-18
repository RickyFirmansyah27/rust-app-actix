use rust_app::routes::create_routes;
use rust_app::helpers::logger::init_logger;
use actix_web::{App, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize tracing
    init_logger();

    // build our application with routes
    let app = create_routes();

    // run our app with actix web
    tracing::debug!("listening on 0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .configure(app)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}