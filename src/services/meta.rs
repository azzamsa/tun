use crate::models::meta as model;

pub async fn meta() -> Result<model::Meta, crate::Error> {
    let meta = model::Meta {
        build_hash: option_env!("BUILD_HASH").unwrap_or("unknown").to_string(),
        build_date: option_env!("BUILD_DATE").unwrap_or("unknown").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(meta)
}
