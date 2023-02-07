use crate::entities::{prelude::User, user};
use crate::schema::DB;
use async_graphql::*;
use sea_orm::*;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        first_name: String,
        last_name: String,
        age: i32,
        is_cool: bool,
    ) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            first_name: ActiveValue::Set(first_name.to_owned()),
            last_name: ActiveValue::Set(last_name.to_owned()),
            age: ActiveValue::Set(age),
            is_cool: ActiveValue::Set(is_cool),
            ..Default::default()
        };

        let db = ctx.data::<DB>().unwrap();
        let connection = db.get_connection();

        let res = User::insert(active_model).exec(connection).await?;

        Ok(user::Model {
            id: res.last_insert_id,
            first_name,
            last_name,
            age,
            is_cool,
        })
    }
}
