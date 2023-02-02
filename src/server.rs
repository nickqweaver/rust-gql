
use warp::Filter;
use std::sync::Arc;
use juniper::{http::graphiql::graphiql_source};
use super::schema::{graphql, build_schema};

pub struct Context;
impl juniper::Context for Context {}


pub async fn start() {
  let schema = build_schema();
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
