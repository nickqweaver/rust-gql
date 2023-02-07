// Module Tree
mod entities;
mod schema;
mod server;
mod user;

use server::start;

#[tokio::main]
async fn main() {
    start().await;
}
