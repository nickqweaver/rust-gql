use crate::server::Context;
use super::{User, Response};
pub struct UserMutation;

graphql_object!(UserMutation: Context |&self| {
    field create(name: String, age: i32, is_cool: bool) -> User {
        User {
            name,
            age,
            is_cool
        }
    }
    field delete(id: i32) -> Response {
      if id > 10 {
        Response {
          success: true,
          message: format!("Successfully deleted user with ID {}", id)        
        }
      } else {
        Response {
          success: false,
          message: format!("Could not find user with {} id", id)        
        }
      }
     
    }
});