use axum::{http::StatusCode, response::IntoResponse};

pub async fn get_statuses() -> impl IntoResponse {
    StatusCode::OK
}
