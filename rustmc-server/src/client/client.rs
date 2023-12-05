use async_trait::async_trait;
use rustmc_errors::{ConnectionError, PacketError};
use rustmc_packets::Packet;
use tokio::net::TcpStream;

use crate::MinecraftServer;

use super::uuid::UUID;

/// A trait representing a client that can connect to a Minecraft server.
#[async_trait]
pub trait Client {
    
    /// Creates a new client instance with the given connection, username, and UUID.
    ///
    /// # Arguments
    ///
    /// * `connection` - The TCP stream connection to the server.
    /// * `username` - The username of the client.
    /// * `uuid` - The UUID of the client.
    ///
    /// # Returns
    ///
    /// A new instance of the client.
    fn new(connection: TcpStream, username: &str, uuid: UUID) -> Self;

    /// Connects the client to the specified Minecraft server.
    ///
    /// # Arguments
    ///
    /// * `server` - The Minecraft server to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the connection was successful or an error occurred.
    async fn connect(&mut self, server: &mut MinecraftServer) -> Result<(), ConnectionError>;

    /// Disconnects the client from the server.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the disconnection was successful or an error occurred.
    fn disconnect(&self);

    /// Sends a packet to the server.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to send.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the packet was sent successfully or an error occurred.
    ///
    /// # Generic
    ///
    /// This method is generic over the type `P`, which must implement the `Packet` trait and be `Sync`.
    async fn send_packet<P>(&mut self, packet: &P)
    where
        P: Packet + Sync;

}