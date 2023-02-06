use std::convert::Infallible;

use async_graphql::{http::GraphiQLSource, EmptySubscription, EmptyMutation, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use warp::{http::Response as HttpResponse, Filter, Rejection};
use http::StatusCode;
use crate::schema::{build_schema, RootQuery};

pub async fn start() {
  let schema = build_schema();

  println!("GraphiQL IDE: http://localhost:8000");

  let graphql_post = async_graphql_warp::graphql(schema).and_then(
    |(schema, request): (
        Schema<RootQuery, EmptyMutation, EmptySubscription>,
        async_graphql::Request,
    )| async move {
        Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
    },
  );

  let graphiql = warp::path::end().and(warp::get()).map(|| {
    HttpResponse::builder()
        .header("content-type", "text/html")
        .body(GraphiQLSource::build().endpoint("/").finish())
  });

  let routes = graphiql
    .or(graphql_post)
    .recover(|err: Rejection| async move {
        if let Some(GraphQLBadRequest(err)) = err.find() {
            return Ok::<_, Infallible>(warp::reply::with_status(
                err.to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(warp::reply::with_status(
            "INTERNAL_SERVER_ERROR".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    });

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
