use std::sync::Arc;

use rustmc_packets::{client::converter::PacketByteConverter, server::handshake::HandshakePacket};
use rustmc_server::{tickable_server::TickableServer, MinecraftServer};
use tokio::signal;

#[tokio::main]
async fn main() {
    let server: Arc<MinecraftServer> = MinecraftServer::new("127.0.0.1", 8080);
    server.start();

    unsafe {
        let converter = PacketByteConverter;
        converter.register_packet(HandshakePacket);

        let packet_test = converter.get_packet(0x00).await;
        println!("{:?}", packet_test);
    }

    signal::ctrl_c().await.unwrap();
}

pub(crate) mod misc;
