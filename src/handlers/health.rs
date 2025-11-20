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
