use crate::server_packet;

server_packet!(0x00, LoginStartPacket {
    name: String,
    player_uuid: String,
});