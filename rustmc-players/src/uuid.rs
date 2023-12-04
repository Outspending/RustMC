#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UUID {
    pub data: [u8; 16],
}