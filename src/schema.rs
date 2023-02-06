use super::user::graphql::{UserMutation, UserQuery};
use async_graphql::*;

pub struct RootQuery;
pub struct RootMutation;

#[Object]
impl RootQuery {
    async fn user(&self) -> UserQuery {
        UserQuery
    }
}

#[Object]
impl RootMutation {
    async fn user(&self) -> UserMutation {
        UserMutation
    }
}

pub fn build_schema() -> Schema<RootQuery, RootMutation, EmptySubscription> {
    let schema = Schema::new(RootQuery, RootMutation, EmptySubscription);
    schema
}
