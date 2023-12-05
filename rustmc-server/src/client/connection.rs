use std::sync::Arc;

use rustmc_errors::PacketError;
use rustmc_packets::Packet;
use tokio::{sync::Mutex, net::TcpStream, io::AsyncWriteExt};

/// Represents a client connection to a server.
#[derive(Debug, Clone)]
pub struct ClientConnection {
    pub connection: Arc<Mutex<TcpStream>>,
}

impl ClientConnection {
    /// Creates a new `ClientConnection` instance.
    ///
    /// # Arguments
    ///
    /// * `connection` - The TCP stream representing the connection.
    ///
    /// # Returns
    ///
    /// A new `ClientConnection` instance.
    pub fn new(connection: TcpStream) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    /// Disconnects the client from the server.
    ///
    /// This method asynchronously shuts down the TCP stream connection.
    pub async fn disconnect(&self) {
        self.connection.lock().await.shutdown();
    }

    /// Sends a packet to the server.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to be sent.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of sending the packet.
    pub async fn send_packet<P>(&self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet
    {
        let connection = self.connection.clone();
        let data = packet.into_protocol_format();
        println!("Sent Packet: {:?}", data.clone()); // TODO: DEBUG

        let mut connection = connection.lock().await;

        match connection.write_all(&data).await {
            Ok(_) => Ok(()),
            Err(_) => Err(PacketError::ErrorSendingPacket),
        }
    }
}