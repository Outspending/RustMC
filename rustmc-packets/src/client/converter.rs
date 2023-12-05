use std::{sync::Arc, collections::HashMap};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::Packet;

/// A vector of boxed packets.
type PacketVec = HashMap<u8, Box<dyn Packet + 'static>>;

/// A vector of boxed packets wrapped in a mutex.
static mut CLIENT_PACKETS: Lazy<Arc<Mutex<PacketVec>>>= Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub struct PacketByteConverter;

impl PacketByteConverter {

    /// Registers a packet for the client.
    ///
    /// This function takes a packet of type `P` that implements the `Packet` trait and registers it for the client.
    /// The packet is boxed and stored in the `CLIENT_PACKETS` collection, along with its ID.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` because it performs unsafe operations, such as dereferencing raw pointers.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to register.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustmc_packets::client::converter::register_packet;
    /// use rustmc_packets::client::Packet;
    ///
    /// struct MyPacket;
    ///
    /// impl Packet for MyPacket {
    ///     fn id(&self) -> u32 {
    ///         42
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     register_packet(MyPacket).await;
    /// }
    /// ```
    pub async unsafe fn register_packet<P>(&self, packet: P)
    where
        P: Packet + 'static
    {
        let packet = Box::new(packet);
        CLIENT_PACKETS.lock().await.insert(packet.id(), packet);
    }

    /// Retrieves a packet from the given `BytesMut` data.
    /// 
    /// # Arguments
    /// 
    /// * `data` - The `BytesMut` data containing the packet.
    /// 
    /// # Returns
    /// 
    /// Returns an `Option<Box<dyn Packet + 'static>>` representing the retrieved packet, or `None` if the packet is not found.
    /// 
    /// # Safety
    /// 
    /// This function is marked as `unsafe` because it performs raw pointer dereferencing.
    pub async unsafe fn get_packet(&self, packet_id: u8) -> Option<Box<dyn Packet + 'static>> {
        return CLIENT_PACKETS.lock().await
            .get(&packet_id)
            .map(|box_packet| (*box_packet).clone());
    }

}