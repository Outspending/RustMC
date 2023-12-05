use crate::server_packet;

server_packet!(0x00, HandshakePacket {
    protocol_version: u16,
    server_address: String,
    server_port: u16,
    next_state: u8,
});