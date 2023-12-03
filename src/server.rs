use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::{
    errors::PacketError,
    players::{Client, Player, UUID},
    protocol::Packet,
};

/// Represents a Minecraft server.
#[derive(Clone)]
pub struct MinecraftServer {
    /// The address of the server.
    pub address: String,
    /// The port number of the server.
    pub port: u16,

    /// The list of players currently connected to the server.
    pub players: Arc<Mutex<Vec<Player>>>,
}

///
/// The main server struct.
///
/// This struct contains everything necessary to run a Minecraft server.
///
/// Start the server by creating a new instance of this struct and calling the `start_server` method.
/// This will automatically start the server and listen for incoming connections.
///
/// Once a connection is established, the `handle_connection` method will be called.
/// And you can send packets to the client by using the `send_packet` method.
///
impl MinecraftServer {
    /// Creates a new instance of the MinecraftServer struct.
    ///
    /// # Arguments
    ///
    /// * `address` - The IP address or hostname to bind the server to.
    /// * `port` - The port number to bind the server to.
    ///
    /// # Returns
    ///
    /// A new instance of the MinecraftServer struct.
    ///
    pub fn new(address: &str, port: u16) -> Arc<Self> {
        Arc::new(Self {
            address: address.to_string(),
            port,
            players: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Starts the server and listens for incoming connections.
    ///
    /// This function binds the server's address and port to a TCP listener and continuously accepts
    /// incoming connections. For each accepted connection, a new task is spawned to handle the
    /// connection using the `handle_connection` function.
    ///
    /// # Arguments
    ///
    /// * `self` - The server instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// let server = MinecraftServer::new("127.0.0.1", 8080);
    /// server.start_server();
    /// ```
    pub fn start_server(&self) {
        let server = self.clone();
        let players_clone = Arc::clone(&self.players);

        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("{}:{}", server.address, server.port))
                .await
                .unwrap();

            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let player = Player {
                            connection: Arc::new(Mutex::new(stream)),
                            username: "wowie".into(),
                            uuid: UUID { data: [0; 16] },
                        };

                        players_clone.lock().unwrap().push(player);
                        tokio::spawn(handle_connection(player, server));
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        });
    }

    /// Returns a vector of players currently connected to the server.
    pub fn get_players(&self) -> Vec<Player> {
        *self.players.lock().unwrap()
    }

    /// Retrieves a player by their username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the player.
    ///
    /// # Returns
    ///
    /// An optional reference to the player if found, otherwise None.
    ///
    pub fn get_player_username(&self, username: &str) -> Option<&Player> {
        self.get_players()
            .iter()
            .find(|&player| player.username.eq(username))
    }

    /// Retrieves a player by their UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID of the player.
    ///
    /// # Returns
    ///
    /// An optional reference to the player if found, otherwise None.
    ///
    pub fn get_player_uuid(&self, uuid: &UUID) -> Option<&Player> {
        self.get_players()
            .iter()
            .find(|&player| player.uuid.eq(uuid))
    }

    /// Sends a packet to a specific player.
    ///
    /// # Arguments
    ///
    /// * `player` - The player to send the packet to.
    /// * `packet` - The packet to send.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of sending the packet.
    ///
    pub async fn send_packet<P>(
        &mut self,
        player: &mut Player,
        packet: &P,
    ) -> Result<(), PacketError>
    where
        P: Packet + Sync,
    {
        player.send_packet(packet).await
    }

    /// Broadcasts a packet to all connected players.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to broadcast.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of broadcasting the packet.
    ///
    pub async fn broadcast_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync,
    {
        for player in self.get_players().iter_mut() {
            player.send_packet(packet).await?;
        }

        Ok(())
    }
}

/// Handles a new connection from a client.
///
/// This function is called when a new TCP connection is established with the server.
/// It creates a new `Player` struct to represent the connected client, initializes its fields,
/// and adds it to the list of players in the `MinecraftServer`.
/// It also prints the IP address of the connected client to the console.
///
/// # Arguments
///
/// * `stream` - The TCP stream representing the connection with the client.
/// * `server` - A mutable reference to the `MinecraftServer` instance.
///
/// # Examples
///
/// ```
/// use std::net::TcpStream;
/// use std::sync::{Arc, Mutex};
/// use crate::server::{MinecraftServer, Player, UUID};
///
/// async fn handle_connection(stream: TcpStream, server: &mut MinecraftServer) {
///     // Implementation omitted for brevity
/// }
/// ```
///
/// # Panics
///
/// This function will panic if it fails to obtain the peer address of the client.
async fn handle_connection(player: Player, server: MinecraftServer) {

    let peer_addr = player.connection.lock().unwrap().peer_addr().unwrap();
    println!("New connection from {}", peer_addr);
}
