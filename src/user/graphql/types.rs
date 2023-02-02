#[derive(GraphQLObject)]
  pub struct User {
    // The users first name
    pub name: String,
    // The users age
    pub age: i32,
    // Is the user cool?
    pub is_cool: bool
  }

// The user isn't actually more complex but when you have fields
// that don't map 1 to 1 to a graphql object you can create a 
// graphql object! and resolve the fields with any additional business logic
pub struct MoreComplexUser {
  pub name: String,
  pub age: i32
}

// This is how we are resolving the fields with extra logic
graphql_object!(MoreComplexUser:() |&self| {
  field name() -> &str {
    self.name.as_str()
  }
  field age() -> i32 {
    self.age + 10
  }
});


