use std::{
    io::{Error, Read, Write},
    os::unix::net::UnixStream,
};

use crate::{
    bytes::ToBytes,
    error::{SocketError, SocketErrorKind},
    request::SocketRequest,
    response::SocketResponse,
    result::Result,
};

pub struct SocketConnection {
    stream: UnixStream,
    blocking: bool,
}

// padding in bytes
const BUFFER_PADDING: usize = 4;

impl SocketConnection {
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self {
            stream,
            blocking: true,
        }
    }

    pub fn handle_par<F>(self, mut handler: F)
    where
        F: FnOnce(SocketConnection) + Send + 'static,
    {
        std::thread::spawn(move || handler(self));
    }

    pub fn recv_msg(&mut self) -> Result<SocketRequest, String> {
        const REQUEST_SIZE: usize = size_of::<SocketRequest>() + BUFFER_PADDING;
        match self.read_stream::<REQUEST_SIZE>() {
            Ok(b) => SocketRequest::from_bytes(&b).map_err(SocketError::from),
            Err(err) => Err(err),
        }
    }

    pub fn post_reply(&mut self, response: SocketResponse) -> Result<(), String> {
        let response_bytes = response.to_bytes();
        self.stream.write_all(response_bytes).map_err(Error::into)
    }

    pub fn post_msg(&mut self, request: SocketRequest) -> Result<SocketResponse, String> {
        let request_bytes = request.to_bytes();
        self.stream.write_all(request_bytes)?;
        self.response()
    }

    pub fn block(&mut self) -> &mut Self {
        if !self.blocking {
            let Ok(_) = self.stream.set_nonblocking(false) else {
                return self;
            };
            self.blocking = true;
        }
        self
    }

    pub fn non_block(&mut self) -> &mut Self {
        if self.blocking {
            let Ok(_) = self.stream.set_nonblocking(true) else {
                return self;
            };
            self.blocking = false;
        }
        self
    }

    fn response(&mut self) -> Result<SocketResponse, String> {
        const RESPONSE_SIZE: usize = size_of::<SocketResponse>() + BUFFER_PADDING; // add some padding
        match self.read_stream::<RESPONSE_SIZE>() {
            Ok(b) => SocketResponse::from_bytes(&b).map_err(SocketError::from),
            Err(err) => Err(err),
        }
    }

    fn read_stream<const SIZE: usize>(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer = [0; SIZE];
        // --- Read bytes to buffer ---
        let read = self.stream.read(&mut buffer[..])?;
        if read + BUFFER_PADDING != SIZE {
            return Err(SocketError::new(
                SocketErrorKind::SocketStreamIO,
                "Invalid bytes size read from stream".to_string(),
                None,
            ));
        }

        if read == 0 {
            return Err(SocketError::new(
                SocketErrorKind::SocketStreamEmpty,
                "Stream is empty".to_string(),
                None,
            ));
        }

        Ok(buffer[..read].to_vec())
    }
}
