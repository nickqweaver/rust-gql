use async_graphql::*;

use super::{User, MoreComplexUser};
pub struct UserQuery;

#[Object]
impl UserQuery {
  async fn get(&self) -> User {
    User {
      name: String::from("Nick Weaver"),
      age: 30,
      is_cool: false
    }
  }
  async fn get_complex(&self) -> MoreComplexUser {
    MoreComplexUser {
      name: String::from("Hello"),
      age: 20 // Should add 10
    }
  }
  async fn find_by_id(&self, id: i32) -> User {
    if id > 10 {
      User {
        name: String::from("Gary"),
        age: 10,
        is_cool: true
      }
    } else {
      User {
        name: String::from("Bary"),
        age: 10,
        is_cool: true
      }
    }
  }
  async fn auth(&self) -> User {
    User {
      name: String::from("I am the AuthUser!"),
      age: 20,
      is_cool: true
    }
  }
} 
