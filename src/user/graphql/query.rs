use crate::schema::DB;
use async_graphql::*;
use sea_orm::{DbErr, EntityTrait};

use super::{FakeUser, MoreComplexUser};
use crate::entities::{prelude::User, user};

#[derive(Default)]
pub struct UserQuery;
pub struct Token(pub String);

#[Object]
impl UserQuery {
    async fn get(&self) -> FakeUser {
        FakeUser {
            name: String::from("Nick Weaver"),
            age: 30,
            is_cool: false,
        }
    }
    async fn get_complex(&self) -> MoreComplexUser {
        MoreComplexUser {
            name: String::from("Hello"),
            age: 20, // Should add 10
        }
    }
    async fn auth(&self) -> FakeUser {
        FakeUser {
            name: String::from("I am the AuthUser!"),
            age: 20,
            is_cool: true,
        }
    }
    pub async fn find_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
    ) -> Result<Option<user::Model>, DbErr> {
        let db = ctx.data::<DB>().unwrap();
        let connection = db.get_connection();

        println!("We can create a fn that looks up the user by the auth token after verification");
        println!("Might be able to just add is_auth_user to the context");
        
        println!("{:?}", ctx.data_opt::<Token>().map(|token| token.0.as_str()));

        Ok(User::find_by_id(id).one(connection).await?)
    }
}
