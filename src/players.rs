use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tokio::{net::TcpStream, io::AsyncWriteExt, runtime::Runtime};

use crate::{
    errors::{ConnectionError, PacketError},
    protocol::Packet,
    server::MinecraftServer,
};

#[async_trait]
pub trait Client {

    fn new(connection: TcpStream, username: &str, uuid: UUID) -> Self;

    fn connect(&mut self, server: MinecraftServer) -> Result<(), ConnectionError>;

    fn disconnect(&mut self) -> Result<(), ConnectionError>;

    async fn send_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync;


}

#[derive(Debug, Clone)]
pub struct Player {
    pub connection: Arc<Mutex<TcpStream>>,

    pub username: String,
    pub uuid: UUID,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UUID {
    pub data: [u8; 16],
}

#[async_trait]
impl Client for Player {

    fn new(connection: TcpStream, username: &str, uuid: UUID) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
            username: username.to_string(),
            uuid,
        }
    }

    fn connect(&mut self, server: MinecraftServer) -> Result<(), ConnectionError> {
        Ok(())
        
    }

    fn disconnect(&mut self) -> Result<(), ConnectionError> {
        Ok(())
    }

    async fn send_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync,
    {
        let connection = Arc::clone(&self.connection);
        let data = packet.into_protocol_format();

        Runtime::new().unwrap().block_on(async {
            let mut connection = connection.lock().unwrap();

            match connection.write_all(&data).await {
                Ok(_) => Ok(()),
                Err(_) => Err(PacketError::ErrorSendingPacket),
            }
        })
    }
}
