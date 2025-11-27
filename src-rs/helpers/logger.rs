use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    tracing_subscriber::EnvFilter::new("actix_web=info,rust_app=debug")
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}