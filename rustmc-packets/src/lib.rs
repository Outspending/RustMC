use bytes::{Buf, BytesMut};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::MutexGuard};

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
        Self: Sized,
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
        P: Packet,
    {
        let data = packet.serialize();
        let mut formatted_data = Vec::new();

        formatted_data.extend_from_slice(&((data.len() + 1) as u8).to_be_bytes()); // Length of Packet ID + Data
        formatted_data.extend_from_slice(&packet.id().to_be_bytes()); // Packet ID
        formatted_data.extend_from_slice(&data); // Data

        data
    }

    /// Reads a variable-length integer from the given buffer.
    ///
    /// This function reads a variable-length integer from the provided `buffer` and returns it along with the number of bytes consumed.
    /// The integer is encoded using a variable number of bytes, with each byte representing 7 bits of the integer value.
    /// The most significant bit (MSB) of each byte indicates whether there are more bytes to follow.
    /// If the buffer does not contain enough bytes to read a complete integer, or if the integer exceeds 5 bytes, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer containing the bytes to read from.
    ///
    /// # Returns
    ///
    /// A tuple containing the parsed integer value and the number of bytes consumed, wrapped in an `Option`.
    /// If the integer cannot be parsed or if the buffer is empty, `None` is returned.
    pub fn read_varint(buffer: &mut BytesMut) -> Option<(usize, usize)> {
        let mut result = 0;
        let mut count = 0;

        loop {
            if count == 5 {
                return None;
            }

            if buffer.len() == 0 {
                return None;
            }

            let byte = buffer.get_u8();
            result |= ((byte & 0xF7) as usize) << (7 * count);

            if (byte & 0x80) == 0 {
                return Some((result, count + 1));
            }

            count += 1;
        }
    }
}

/// Represents a packet retriever.
pub struct PacketRetriever;

/// This struct represents a packet retriever, responsible for retrieving packets from a TCP connection.
/// It provides methods for creating a new instance and asynchronously retrieving packets from the connection.
impl PacketRetriever {
    /// Asynchronously retrieves packets from the player's connection.
    ///
    /// This function reads data from the player's connection in a loop until no more data is available.
    /// The read data is stored in a buffer and processed as packets using the `PacketFormatter` struct.
    /// The processed packets are then passed to the `process_packet` function for further handling.
    ///
    /// # Arguments
    ///
    /// * `player` - A reference to the `Player` struct representing the player.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::protocol::PacketHandler;
    /// use crate::player::Player;
    ///
    /// let packet_handler = PacketHandler::new();
    /// let player = Player::new();
    ///
    /// packet_handler.retrieve_packets(&player).await;
    /// ```
    pub async fn retrieve_packets(connection: &mut MutexGuard<'_, TcpStream>) {
        let mut buffer = BytesMut::with_capacity(1024);

        loop {
            let mut read_buffer: [u8; 1024] = [0; 1024];
            match connection.read(&mut read_buffer).await {
                Ok(bytes_read) => {
                    println!("Bytes read: {}", bytes_read);

                    if bytes_read == 0 {
                        break;
                    }

                    buffer.extend_from_slice(&read_buffer[..bytes_read]);

                    if let Some((length, _)) = PacketFormatter::read_varint(&mut buffer) {
                        if buffer.len() >= length {
                            let packet_data = buffer.split_to(length);
                            PacketRetriever::process_packet(packet_data).await;
                        } else {
                            break;
                        }
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Error reading from TcpStream from connection: {:?}",
                        connection
                    );
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    pub async fn process_packet(packet_data: BytesMut) {
        println!("Received Packet: {:?}", packet_data);

        let packet_id = packet_data.first().map(|&byte| byte).unwrap();
        println!("Received Packet ID: {}", packet_id);
    }
}

pub(crate) mod client;
pub(crate) mod server;
pub(crate) mod macros;
