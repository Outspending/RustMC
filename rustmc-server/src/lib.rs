use std::{
    cell::{Ref, RefCell},
    process,
    sync::Arc,
    time::Instant,
};

use async_trait::async_trait;
use client::{client::Client, uuid::UUID, Player};
use rustmc_errors::PacketError;
use rustmc_packets::{Packet, PacketRetriever};
use tickable_server::TickableServer;
use tokio::{net::TcpListener, sync::Mutex};

/// Represents a Minecraft server.
#[derive(Clone)]
pub struct MinecraftServer {
    /// The address of the server.
    pub address: String,
    /// The port number of the server.
    pub port: u16,

    /// The list of players currently connected to the server.
    pub players: RefCell<Vec<Player>>,
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
#[async_trait]
impl TickableServer for MinecraftServer {
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
    fn new(address: &str, port: u16) -> Arc<Self> {
        Arc::new(Self {
            address: address.to_string(),
            port,
            players: RefCell::new(Vec::new()),
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
    /// server.start();
    /// ```
    fn start(&self) {
        let start = Instant::now();
        let server = self.clone();

        println!("Starting RustMC Server.");
        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("{}:{}", server.address, server.port)).await;

            match listener {
                Ok(listener) => loop {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            let mut server_clone = server.clone();
                            let mut player = Player {
                                connection: Arc::new(Mutex::new(stream)),
                                username: "wowie".into(),
                                uuid: UUID { data: [0; 16] },
                            };

                            server.players.borrow_mut().push(player.clone());
                            tokio::spawn(async move {
                                handle_connection(&mut player, &mut server_clone).await;
                            });
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error while trying to start RustMC Server: {}", e);
                    return;
                }
            }
        });

        println!("Started RustMC Server in {:?}", start.elapsed());
    }

    /// Stops the server by closing the TCP listener and disconnecting all players.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::process;
    /// use tokio::net::TcpListener;
    ///
    /// # struct Player {
    /// #     // ...
    /// # }
    /// #
    /// # impl Player {
    /// #     fn disconnect(&self, message: &str) {
    /// #         // ...
    /// #     }
    /// # }
    /// #
    /// # struct Server {
    /// #     address: String,
    /// #     port: u16,
    /// # }
    /// #
    /// # impl Server {
    /// #     fn get_players(&self) -> Vec<Player> {
    /// #         // ...
    /// #     }
    /// #
    /// #     fn force_stop(&self) {
    /// #         // ...
    /// #     }
    /// #
    ///     pub async fn stop(&self) {
    ///         let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).await;
    ///
    ///         match listener {
    ///             Ok(listener) => {
    ///
    ///                 for player in self.get_players().iter() {
    ///                     player.disconnect("Server is shutting down.");
    ///                 }
    ///
    ///                 process::exit(0);
    ///             }
    ///             Err(e) => {
    ///                 eprintln!("There was an error while trying to stop the server: {}", e);
    ///                 eprintln!("Force stopping the server...");
    ///                 self.force_stop();
    ///             }
    ///         }
    ///     }
    /// # }
    /// ```
    fn stop(&self) {
        for player in self.get_players().iter() {
            player.disconnect("Server is shutting down.");
        }

        process::exit(0);
    }

    /// Forces the server to stop by exiting the process with code 0.
    fn force_stop(&self) {
        process::exit(0);
    }

    /// Returns a vector of players currently connected to the server.
    fn get_players(&self) -> Ref<'_, Vec<Player>> {
        self.players.borrow()
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
    fn get_player_username(&self, username: &str) -> Option<Player> {
        self.get_player_filter(|player| player.username == username)
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
    fn get_player_uuid(&self, uuid: UUID) -> Option<Player> {
        self.get_player_filter(|player| player.uuid == uuid)
    }

    /// Returns an optional player that matches the given filter function.
    ///
    /// # Arguments
    ///
    /// * `filter` - A closure that takes a reference to a `Player` and returns a `bool`.
    ///
    /// # Returns
    ///
    /// * `Some(player)` - If a player is found that matches the filter function.
    /// * `None` - If no player is found that matches the filter function.
    fn get_player_filter(&self, filter: impl Fn(&Player) -> bool) -> Option<Player> {
        for player in self.get_players().iter() {
            if filter(player) {
                return Some(player.clone());
            }
        }

        None
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
    async fn broadcast_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync,
    {
        unimplemented!()
    }

    async fn send_server_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync,
    {
        unimplemented!()
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
async fn handle_connection(player: &mut Player, server: &mut MinecraftServer) {
    player.connect(server).await.unwrap();

    let mut connection = player.connection.lock().await;
    let peer_addr = connection.peer_addr().unwrap();
    println!("New connection from {}", peer_addr);

    PacketRetriever::retrieve_packets(&mut connection).await;
}

pub mod client;
pub mod tickable_server;
