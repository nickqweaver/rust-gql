use std::convert::Infallible;

use crate::schema::{build_schema, RootMutation, RootQuery};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};
use crate::user::graphql::Token;

pub async fn start() {
    let schema = build_schema().await;

    println!("GraphiQL IDE: http://localhost:8000");

    let graphql_post = warp::header::optional::<String>("Authorization")
        .and(async_graphql_warp::graphql(schema))
        .and_then(
            |token,
             (schema, mut request): (
                Schema<RootQuery, RootMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                if let Some(token) = token {
                    println!("There was a token boys {}", token);
                    request = request.data(Token(token));
                }
                //let resp = schema.execute(request).await;

                let response = schema.execute(request).await;
                Ok::<_, Infallible>(GraphQLResponse::from(response))
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
