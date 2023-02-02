// Module Tree
mod user;
mod schema;
mod server;

use server::start;

#[macro_use] extern crate juniper;

// CONTEXST
pub struct Context;
impl juniper::Context for Context {}



#[tokio::main]
async fn main() {
    start().await;
}