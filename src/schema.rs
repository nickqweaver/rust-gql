use std::sync::Arc;
use std::convert::Infallible;
use juniper::{http::GraphQLRequest, RootNode};
use super::user::graphql::{UserQuery, UserMutation};
use crate::server::Context;


pub struct RootQuery;
pub struct RootMutation;

graphql_object!(RootQuery: Context |&self| {
  field user() -> UserQuery { UserQuery }
});

graphql_object!(RootMutation: Context |&self| {
  field user() -> UserMutation { UserMutation }
});

pub type Schema = RootNode<'static, RootQuery, RootMutation>;

pub fn build_schema() -> Arc<Schema> {
  let schema = Arc::new(Schema::new(RootQuery, RootMutation));
  schema
}

pub async fn graphql(
  schema: Arc<Schema>,
  ctx: Arc<Context>,
  req: GraphQLRequest,
) -> Result<impl warp::Reply, Infallible> {
  let res = req.execute(&schema, &ctx);
  let json = serde_json::to_string(&res).expect("Invalid JSON response");
  Ok(json)
}