
use warp::Filter;
use std::sync::Arc;
use std::convert::Infallible;
use juniper::{http::GraphQLRequest, http::graphiql::graphiql_source, RootNode};

// Module Tree
mod user;

use user::graphql::{query::UserQuery, mutation::UserMutation};

#[macro_use] extern crate juniper;

// CONTEXST
pub struct Context;
impl juniper::Context for Context {}

type Schema = RootNode<'static, UserQuery, UserMutation>;

async fn graphql(
    schema: Arc<Schema>,
    ctx: Arc<Context>,
    req: GraphQLRequest,
) -> Result<impl warp::Reply, Infallible> {
    let res = req.execute(&schema, &ctx);
    let json = serde_json::to_string(&res).expect("Invalid JSON response");
    Ok(json)
}

#[tokio::main]
async fn main() {
    let schema = Arc::new(Schema::new(UserQuery, UserMutation));
    // Create a warp filter for the schema
    let schema = warp::any().map(move || Arc::clone(&schema));

    let ctx = Arc::new(Context);
    // Create a warp filter for the context
    let ctx = warp::any().map(move || Arc::clone(&ctx));

    let graphql_route = warp::post()
        .and(warp::path!("graphql"))
        .and(schema.clone())
        .and(ctx.clone())
        .and(warp::body::json())
        .and_then(graphql);

    let graphiql_route = warp::get()
        .and(warp::path!("graphiql"))
        .map(|| warp::reply::html(graphiql_source("graphql")));

    let routes = graphql_route.or(graphiql_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}