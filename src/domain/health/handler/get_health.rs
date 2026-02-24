#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "health status"),
    ),
)]
pub async fn get_health() -> Result<(), crate::Error> {
    Ok(())
}
