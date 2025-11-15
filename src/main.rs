use rust_app::routes::create_routes;
use rust_app::helpers::logger::init_logger;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    init_logger();

    // build our application with routes
    let app = create_routes();

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}