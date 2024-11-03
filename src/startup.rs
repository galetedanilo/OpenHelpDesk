use axum::Router;

use crate::{configuration::Settings, web::status::routes::status_routes};

pub struct Application {}

impl Application {
    pub async fn run() {
        let settings = Settings::init().expect("Failed to read configuration");

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let api_version = format!("/api/{}", settings.application.version);

        let api_routes = Router::new().merge(status_routes());

        let app = Router::new().nest(&api_version, api_routes);

        let listener = tokio::net::TcpListener::bind(address).await.unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}
