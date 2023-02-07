use async_graphql::*;

#[derive(SimpleObject)]
pub struct FakeUser {
    // The users first name
    pub name: String,
    // The users age
    pub age: i32,
    // Is the user cool?
    pub is_cool: bool,
}

#[derive(SimpleObject)]
pub struct Response {
    pub success: bool,
    pub message: String,
}

// The user isn't actually more complex but when you have fields
// that don't map 1 to 1 to a graphql object you can create a
// graphql object! and resolve the fields with any additional business logic
pub struct MoreComplexUser {
    pub name: String,
    pub age: i32,
}

// This is how we are resolving the fields with extra logic
#[Object]
impl MoreComplexUser {
    async fn name(&self) -> &str {
        self.name.as_str()
    }
    async fn age(&self) -> i32 {
        self.age + 10
    }
}
