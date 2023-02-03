// Module Tree
mod user;
mod schema;
mod server;

use server::start;

#[tokio::main]
async fn main() {
    start().await;
}