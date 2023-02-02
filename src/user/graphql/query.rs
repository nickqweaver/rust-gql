use super::types::{User, MoreComplexUser};
use crate::Context;
pub struct UserQuery;

// This would be consider our "Root" Query for the User App
// Still needing to combined Queries into a Single Root Query
graphql_object! (UserQuery: Context |&self| {
  field get_user() -> User {
      User {
          name: String::from("Nick Weaver"),
          age: 30,
          is_cool: false
      }
  }
  field get_complex_user() -> MoreComplexUser {
    return MoreComplexUser {
      name: String::from("Hello"),
      age: 20 // Should add 10
    }
  }
});

