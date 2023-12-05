use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::Packet;

/// A vector of boxed packets.
type PacketVec = Vec<(u8, Box<dyn Packet + 'static>)>;

/// A vector of boxed packets wrapped in a mutex.
static mut CLIENT_PACKETS: Lazy<Arc<Mutex<PacketVec>>>= Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

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
    pub async unsafe fn register_packet<P>(packet: P)
    where
        P: Packet + 'static
    {
        let packet = Box::new(packet);
        CLIENT_PACKETS.lock().await.push((packet.id(), packet));
    }

    pub async fn get_packet<P>(data: Vec<u8>) -> Option<P>
    where
        P: Packet + 'static
    {
        todo!("Implement PacketByteConverter::get_packet()") // TODO: Implement PacketByteConverter::get_packet()
    }

}