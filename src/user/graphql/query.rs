use super::{User, MoreComplexUser};
use crate::server::Context;

pub struct UserQuery;

// This would be consider our "Root" Query for the User App
graphql_object! (UserQuery: Context |&self| {
  field get() -> User {
      User {
          name: String::from("Nick Weaver"),
          age: 30,
          is_cool: false
      }
  }
  field get_complex() -> MoreComplexUser {
    return MoreComplexUser {
      name: String::from("Hello"),
      age: 20 // Should add 10
    }
  }
  field find_by_id(id: i32) -> User {
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
  field auth() -> User {
    User {
      name: String::from("I am the AuthUser!"),
      age: 20,
      is_cool: true
    }
  }
});

