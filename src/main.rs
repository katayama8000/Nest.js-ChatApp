mod api;
mod command;
mod domain;
mod mutation_root;
mod query;
mod query_root;
mod server;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::error::Error;

use crate::{mutation_root::Mutation, query_root::Query};

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    let shared_state = server::new()?;
    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/version", get(version))
        .with_state(shared_state); // Wrap the version function inside a closure
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn version() -> impl IntoResponse {
    env!("CARGO_PKG_VERSION")
}
