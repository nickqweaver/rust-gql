use crate::Context;
use super::types::User;
pub struct UserMutation;

graphql_object!(UserMutation: Context |&self| {
    field createUser(name: String, age: i32, is_cool: bool) -> User {
        User {
            name,
            age,
            is_cool
        }
    }
});