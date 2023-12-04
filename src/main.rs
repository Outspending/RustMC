use std::sync::Arc;

use server::MinecraftServer;
use tokio::signal;

#[tokio::main]
async fn main() {
    let server: Arc<MinecraftServer> = MinecraftServer::new("127.0.0.1", 8080);
    println!("Starting Server.");

    server.start_server();
    println!("Server started.");

    signal::ctrl_c().await.unwrap();
}

pub(crate) mod errors;
pub(crate) mod players;
pub(crate) mod protocol;
pub(crate) mod server;
pub(crate) mod misc;
