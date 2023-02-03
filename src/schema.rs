use async_graphql::*;
use super::user::graphql::{UserQuery};

pub struct RootQuery;
// pub struct RootMutation;

#[Object]
impl RootQuery {
  async fn user(&self) -> UserQuery {UserQuery}
}

// #[Object]
// impl RootMutation {
//   async fn user(&self) -> UserMutation {UserMutation}
// }

pub fn build_schema() -> Schema<RootQuery, EmptyMutation, EmptySubscription> {
  let schema = Schema::new(RootQuery, EmptyMutation, EmptySubscription);
  schema
}
