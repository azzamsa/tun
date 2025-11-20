use utoipa_axum::{router::OpenApiRouter, routes};

pub(crate) fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(health))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "health status"),
    ),
)]
pub async fn health() -> Result<(), crate::Error> {
    Ok(())
}
