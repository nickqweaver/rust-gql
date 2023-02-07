use super::user::graphql::{UserMutation, UserQuery};
use async_graphql::*;
use sea_orm::{Database, DatabaseConnection};

#[derive(MergedObject, Default)]
pub struct RootQuery(UserQuery);

#[derive(MergedObject, Default)]
pub struct RootMutation(UserMutation);

pub struct DB {
    pub connection: DatabaseConnection,
}

const DATABASE_URL: &str = "";

impl DB {
    pub async fn new() -> Self {
        let connection = Database::connect(DATABASE_URL)
            .await
            .expect("Could not connect to database");

        DB { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

pub async fn build_schema() -> Schema<RootQuery, RootMutation, EmptySubscription> {
    let db = DB::new().await;

    let schema = Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
    .data(db)
    .finish();
    schema
}
