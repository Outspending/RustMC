///
/// This module contains everything related to the Minecraft protocol.
/// 
/// This trait is used for all packets.
/// 
pub trait Packet {

    /// 
    /// Returns the ID of the packet.
    /// 
    fn id(&self) -> u8;

    /// 
    /// This method serializes the packet into a Vec<u8> which is an array of bytes.
    /// This helps the API to send the packet to the server / client.
    /// 
    fn serialize(&self) -> Vec<u8>;

    ///
    /// This method deserializes the packet from a Vec<u8> which is an array of bytes.
    /// This helps the API to receive the packet from the server / client.
    /// 
    fn deserialize(data: Vec<u8>) -> Option<Self>
    where
        Self: Sized;

    /// 
    /// This method is used to convert the packet into the format used by Minecraft's protocol.
    /// 
    fn into_protocol_format(&self) -> Vec<u8>
    where
        Self: Sized
    {
        PacketFormatter::format_data(self)
    }
}

///
/// This struct is used to create a Packet with the correct format by Minecraft's protocol.
/// 
#[derive(Debug, Clone)]
pub struct PacketFormatter;

/// 
/// PacketFormatter is used to create a new Packet with the correct format by Minecraft's protocol.
/// 
impl PacketFormatter {

    ///
    /// Used to get the format of the PacketFormatter.
    /// This method is essential for sending packets to the server / client.
    /// 
    /// This method will return a Vec<u8> which will contain everything nessessary to send the packet.
    /// 
    /// wiki.vg (Packet Format - Without Compression):
    /// | Length    | VarInt     | Length of Packet ID + Data
    /// | Packet ID | VarInt     | 
    /// | Data      | Byte Array | Depends on the connection state and packet ID, see the sections below
    /// 
    pub fn format_data<P>(packet: &P) -> Vec<u8> 
    where
        P: Packet
    {
        let data = packet.serialize();
        let mut formatted_data = Vec::new();

        formatted_data.extend_from_slice(&((data.len() + 1) as u8).to_be_bytes());   // Length of Packet ID + Data
        formatted_data.extend_from_slice(&packet.id().to_be_bytes());                // Packet ID
        formatted_data.extend_from_slice(&data);                                     // Data

        data
    }

}

pub(crate) mod macros;
pub(crate) mod client;
pub(crate) mod server;