use std::sync::Arc;
use std::time::Duration;

use crate::players::UUID;
use crate::protocol::{server::handshake::HandshakePacket, Packet};
use server::MinecraftServer;
use tokio::signal;

#[tokio::main]
async fn main() {
    let server: Arc<MinecraftServer> = MinecraftServer::new("127.0.0.1", 8080);
    println!("Starting Server.");

    server.start_server();
    println!("Server started.");

    packet_testing().await;

    tokio::spawn(async move {
        loop {
            println!("Players: {:?}", server.players);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    signal::ctrl_c().await.unwrap();
}

async fn packet_testing() {
    let packet = HandshakePacket {
        protocol_version: 271,
        server_address: "127.0.0.1".to_string(),
        server_port: 8080,
        next_state: 0x01,
    };

    let data: Vec<u8> = packet.serialize();
    let packet: HandshakePacket = HandshakePacket::deserialize(data.clone()).unwrap();
    println!("Serialized packet: {:?}", data);
    println!("Protocol packet: {:?}", packet.into_protocol_format());
}

pub(crate) mod players;
pub(crate) mod protocol;
pub(crate) mod server;
pub(crate) mod errors;