use std::sync::Arc;

use async_trait::async_trait;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::{
    errors::{ConnectionError, PacketError},
    protocol::{server::handshake::HandshakePacket, Packet},
    server::MinecraftServer,
};

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
    fn disconnect(&self) -> Result<(), ConnectionError>;

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
    async fn send_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync;
}

/// Represents a player in the game.
#[derive(Debug, Clone)]
pub struct Player {
    /// The connection of the player.
    pub connection: Arc<Mutex<TcpStream>>,

    /// The username of the player.
    pub username: String,

    /// The UUID (Universally Unique Identifier) of the player.
    pub uuid: UUID,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UUID {
    pub data: [u8; 16],
}

#[async_trait]
impl Client for Player {
    /// Creates a new instance of the `Player` struct.
    ///
    /// # Arguments
    ///
    /// * `connection` - The TCP stream connection for the player.
    /// * `username` - The username of the player.
    /// * `uuid` - The UUID of the player.
    ///
    /// # Returns
    ///
    /// A new instance of the `Player` struct.
    fn new(connection: TcpStream, username: &str, uuid: UUID) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
            username: username.to_string(),
            uuid,
        }
    }

    /// Connects the player to the Minecraft server.
    ///
    /// # Arguments
    ///
    /// * `server` - A mutable reference to the Minecraft server.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the connection is successful, otherwise returns a `ConnectionError`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::MinecraftServer;
    /// # use crate::ConnectionError;
    /// # use crate::HandshakePacket;
    /// # struct Player {
    /// #     username: String,
    /// # }
    /// # impl Player {
    /// #     async fn send_packet(&mut self, packet: &HandshakePacket) {}
    /// # }
    /// # async fn example() -> Result<(), ConnectionError> {
    /// #     let mut server = MinecraftServer::new();
    /// #     let mut player = Player {
    /// #         username: String::from("player1"),
    /// #     };
    ///     player.connect(&mut server)?;
    ///     Ok(())
    /// # }
    /// ```
    async fn connect(&mut self, server: &mut MinecraftServer) -> Result<(), ConnectionError> {
        println!("Player {} connected to server.", self.username);

        self.send_packet(&HandshakePacket {
            protocol_version: 764, // 1.20.2
            server_address: server.address.clone(),
            server_port: server.port.clone(),
            next_state: 0x02, // Login
        })
        .await
        .unwrap();

        Ok(())
    }

    /// Disconnects the player from the server.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the disconnection was successful.
    /// Returns an `Err` variant if there was an error during the disconnection process.
    fn disconnect(&self) -> Result<(), ConnectionError> {
        Ok(())
    }

    /// Sends a packet over the network connection.
    ///
    /// This method takes a reference to a packet implementing the `Packet` trait and sends it over the network connection.
    /// The packet must also implement the `Sync` trait to ensure it can be safely shared between threads.
    /// The method returns a `Result` indicating whether the packet was successfully sent or an error occurred.
    ///
    /// # Arguments
    ///
    /// * `packet` - A reference to the packet to be sent.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::players::Player;
    /// use crate::network::Packet;
    ///
    /// let player = Player::new();
    /// let packet = player.create_packet();
    /// let result = player.send_packet(&packet);
    ///
    /// match result {
    ///     Ok(()) => println!("Packet sent successfully"),
    ///     Err(err) => println!("Error sending packet: {:?}", err),
    /// }
    /// ```
    async fn send_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync,
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
