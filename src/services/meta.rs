use crate::models::meta as model;

pub async fn meta() -> Result<model::Meta, crate::Error> {
    let meta = model::Meta {
        build: option_env!("VCS_REVISION").unwrap_or("unknown").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(meta)
}
