use axum::{routing::get, Router};
// startup.rs
use crate::configuration::Settings;

pub struct Application {}

impl Application {
    pub async fn build() {
        let settings = Settings::init().expect("Failed to read configuration");

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        run(address).await;
    }
}

async fn run(address: String) {
    let app = Router::new().route("/", get(|| async { "Hello word" }));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
