use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
};
#[cfg(feature = "metrics")]
use fuel_metrics::core_metrics::encode_metrics_response;

pub async fn metrics(_req: Request<Body>) -> impl IntoResponse {
    #[cfg(feature = "metrics")]
    {
        encode_metrics_response()
    }
    #[cfg(not(feature = "metrics"))]
    {
        use axum::http::StatusCode;
        (StatusCode::NOT_FOUND, "metrics collection disabled")
    }
}
