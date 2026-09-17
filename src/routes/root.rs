use axum::{Json, extract::State, http::header, response::IntoResponse};
use tracing::debug;

use crate::{model::config::DomainFilter, routes::WEBHOOK_CONTENT_TYPE};

#[axum::debug_handler]
pub async fn get_root(
    State(domain_filter): State<DomainFilter>
) -> impl IntoResponse {
    debug!("domain_filter: {:?}", &domain_filter);
    (
        [(header::CONTENT_TYPE, WEBHOOK_CONTENT_TYPE)],
        Json(domain_filter)
    )
}
