#[derive(Debug)]
pub enum PacketError {
    InvalidPacketID,
    InvalidPacketData,
    ErrorFormattingPacket,
    ErrorSendingPacket,
}

#[derive(Debug)]
pub enum ConnectionError {
    InvalidHandshake,
    InvalidLogin,
    InvalidStatus,
    InvalidPlay,
}