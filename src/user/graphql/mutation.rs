use super::{Response, User};
use async_graphql::*;

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create(&self, name: String, age: i32, is_cool: bool) -> User {
        User { name, age, is_cool }
    }
    async fn delete(&self, id: i32) -> Response {
        if id > 10 {
            Response {
                success: true,
                message: format!("Successfully deleted user with ID {}", id),
            }
        } else {
            Response {
                success: false,
                message: format!("Could not find user with {} id", id),
            }
        }
    }
}
