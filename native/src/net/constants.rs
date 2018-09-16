#[repr(u8)]
pub enum MsgType {
    NONE = 0,
    HANDSHAKE = 1,
    PROPERTIES = 2
}
