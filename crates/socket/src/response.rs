// #[repr(packed)]

use std::fmt::Debug;

use super::bytes::ToBytes;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SocketResponse {
    status: Status,
}

//TODO: Allow for dynamic responses probably implement macro
#[allow(dead_code)]
struct SocketResponseHeader {
    length: usize,
}

impl SocketResponse {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
}

impl ToBytes<SocketResponse> for SocketResponse {}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Accepted,
    Rejected,
}
