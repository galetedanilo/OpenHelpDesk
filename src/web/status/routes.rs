use axum::{routing::get, Router};

use super::handler::get_statuses;

pub fn status_routes() -> Router {
    let routers = Router::new().route("/", get(get_statuses));

    Router::new().nest("/statuses", routers)
}
