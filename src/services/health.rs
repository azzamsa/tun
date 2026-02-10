use crate::models::health as model;

pub async fn health() -> Result<model::Health, crate::Error> {
    let health = model::Health {
        status: "running".to_string(),
    };
    Ok(health)
}
