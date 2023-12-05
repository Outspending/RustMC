use crate::packet;

packet!(0x00, LoginStartPacket {
    name: String,
    player_uuid: String,
});