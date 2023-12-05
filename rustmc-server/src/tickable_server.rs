use std::{cell::Ref, sync::Arc};

use async_trait::async_trait;
use rustmc_errors::PacketError;
use rustmc_packets::Packet;

use crate::client::{uuid::UUID, Player};

/// A trait representing a tickable server.
#[async_trait]
pub trait TickableServer {
    /// Creates a new instance of the server with the specified address and port.
    ///
    /// # Arguments
    ///
    /// * `address` - The server address.
    /// * `port` - The server port.
    ///
    /// # Returns
    ///
    /// Returns an `Arc` pointer to the new server instance.
    fn new(address: &str, port: u16) -> Arc<Self>;

    /// Starts the server.
    fn start(&self);

    /// Stops the server gracefully.
    fn stop(&self);

    /// Forces the server to stop immediately.
    fn force_stop(&self);

    /// Returns a reference to the list of players on the server.
    fn get_players(&self) -> Ref<'_, Vec<Player>>;

    /// Returns an `Option` containing the player with the specified username, if found.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the player.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the player with the specified username, if found.
    fn get_player_username(&self, username: &str) -> Option<Player>;

    /// Returns an `Option` containing the player with the specified UUID, if found.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID of the player.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the player with the specified UUID, if found.
    fn get_player_uuid(&self, uuid: UUID) -> Option<Player>;

    /// Returns an `Option` containing the player that matches the specified filter function, if found.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter function to match the player.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the player that matches the specified filter function, if found.
    fn get_player_filter(&self, filter: impl Fn(&Player) -> bool) -> Option<Player>;

    /// Broadcasts a packet to all connected players asynchronously.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to broadcast.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the broadcast was successful or not.
    async fn broadcast_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync;

    /// Sends a packet to the server asynchronously.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to send.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the send was successful or not.
    async fn send_server_packet<P>(&mut self, packet: &P) -> Result<(), PacketError>
    where
        P: Packet + Sync;
}
