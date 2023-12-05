use std::sync::Arc;

use rustmc_server::{MinecraftServer, tickable_server::TickableServer};
use tokio::signal;

#[tokio::main]
async fn main() {
    let server: Arc<MinecraftServer> = MinecraftServer::new("127.0.0.1", 8080);
    server.start();

    signal::ctrl_c().await.unwrap();
}

pub(crate) mod misc;
