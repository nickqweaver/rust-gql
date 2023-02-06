// Module Tree
mod schema;
mod server;
mod user;

use server::start;

#[tokio::main]
async fn main() {
    start().await;
}
