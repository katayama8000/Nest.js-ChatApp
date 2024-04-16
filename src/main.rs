mod mutation;
mod query;

use std::error::Error;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};

use crate::{mutation::Mutation, query::Query};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create the schema
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
