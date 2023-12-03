pub enum PacketError {
    InvalidPacketID,
    InvalidPacketData,
    ErrorFormattingPacket,
    ErrorSendingPacket,
}

pub enum ConnectionError {
    InvalidHandshake,
    InvalidLogin,
    InvalidStatus,
    InvalidPlay,
}