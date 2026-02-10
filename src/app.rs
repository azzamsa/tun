use std::{fs, sync::Arc};

use async_graphql::{
    EmptySubscription, MergedObject, Schema,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{self, IntoResponse},
    routing,
};
use sea_orm as orm;

use crate::{
    Error, config,
    handlers::{
        github::GithubQuery,
        health::HealthQuery,
        meta::MetaQuery,
        user::{UserMutation, UserQuery},
    },
};

pub(crate) struct ServerContext {
    pub db: sea_orm::DatabaseConnection,
    pub config: config::Config,
}

#[derive(MergedObject, Default)]
struct Query(MetaQuery, HealthQuery, GithubQuery, UserQuery);

#[derive(MergedObject, Default)]
struct Mutation(UserMutation);

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn create(
    db: orm::DatabaseConnection,
    config: config::Config,
) -> Result<Router, crate::Error> {
    let is_production = config.env == config::Env::Production;
    let context = Arc::new(ServerContext {
        config: config.clone(),
        db,
    });

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::clone(&context))
        .finish();

    if config.schema_location.is_some() {
        export_schema(&config, &schema).await?;
    }

    let mut router = Router::new().route("/graphql", routing::post(graphql_handler));

    if !is_production {
        router = router.route("/playground", routing::get(graphql_playground));
    }

    Ok(router.with_state(schema))
}

async fn graphql_handler(State(schema): State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn db(config: &config::Config) -> Result<orm::DatabaseConnection, Error> {
    Ok(orm::Database::connect(&config.database_url).await.unwrap())
}

async fn export_schema(config: &config::Config, schema: &AppSchema) -> Result<(), Error> {
    if let Some(location) = &config.schema_location {
        fs::write(location, schema.sdl()).map_err(|_| {
            Error::InvalidArgument(format!(
                "GraphQL schema location doesn't exists `{}`",
                &location.display()
            ))
        })?;
        tracing::info!("Wrote GraphQL schema to {}", location.display());
    }
    Ok(())
}
