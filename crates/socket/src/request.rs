use std::fmt::Debug;

use super::bytes::ToBytes;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SocketRequest {
    msg: Message,
}

impl SocketRequest {
    pub fn new(msg: Message) -> Self {
        Self { msg }
    }

    pub fn is_break(&self) -> bool {
        matches!(self.msg, Message::Break)
    }

    pub fn is_stop(&self) -> bool {
        matches!(self.msg, Message::Stop)
    }
}

impl ToBytes<SocketRequest> for SocketRequest {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Break,
    Stop,
    Forward,
    Unknown,
}
