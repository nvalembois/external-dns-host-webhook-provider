use axum::response::IntoResponse;
use tracing::debug;

#[axum::debug_handler]
pub async fn get_healthz() -> impl IntoResponse {
    debug!("get_health");
    "Ok!"
}
