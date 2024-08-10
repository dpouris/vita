use std::{
    os::unix::net::{SocketAddr, UnixListener, UnixStream},
    path::Path,
    process::exit,
    sync::mpsc::Receiver,
};

use crate::{
    connection::SocketConnection,
    error::{SocketError, SocketErrorKind},
    result::Result,
    signals::{Action, Signal},
};

pub struct SocketClient<'sc> {
    listener: Option<UnixListener>,
    pub socket_path: &'sc str,
}

pub struct IncomingSocketConnection<'sc> {
    sc: &'sc SocketClient<'sc>,
}

impl<'sc> SocketClient<'sc> {
    pub fn new(socket_path: &'sc str) -> Self {
        Self {
            listener: None,
            socket_path,
        }
    }

    pub fn terminate(socket_path: &str) -> Result<(), String> {
        println!("Terminating socket...");
        std::fs::remove_file(socket_path).map_err(SocketError::from)
    }

    pub fn bind(&mut self) -> Result<(), String> {
        let socket_listener =
            UnixListener::bind(self.socket_path).or_else(|err| match err.kind() {
                std::io::ErrorKind::AlreadyExists => self.rebind(),
                _ => Err(SocketError::from(err)),
            })?;

        println!(
            "Bind on socket `{socket_path}`",
            socket_path = self.socket_path
        );

        self.listener = Some(socket_listener);
        Ok(())
    }

    fn rebind(&self) -> Result<UnixListener, String> {
        println!(
            "`{socket}` already exists. Cleaning up leftover socket...",
            socket = self.socket_path
        );
        Self::terminate(self.socket_path)?;
        UnixListener::bind(self.socket_path).map_err(SocketError::from)
    }

    pub fn connect(&self) -> Result<SocketConnection, String> {
        let socket_path = Path::new(self.socket_path);
        if !socket_path.exists() {
            return Err(SocketError::new(
                SocketErrorKind::SocketInvalidPath,
                format!("`{path}` does not exist", path = socket_path.display()),
                None,
            ));
        }

        let socket_addr = SocketAddr::from_pathname(socket_path)?;
        let stream = UnixStream::connect_addr(&socket_addr)?;

        Ok(SocketConnection::new(stream))
    }

    pub fn on_recv(&self, rx: Receiver<()>, action: Action) {
        let socket_path = self.socket_path.to_owned();
        std::thread::spawn(move || {
            if rx.recv().is_ok() {
                // TODO: handle result
                match action {
                    Action::Terminate => {
                        _ = Self::terminate(&socket_path);
                    }
                    Action::TerminateExit(code) => {
                        _ = Self::terminate(&socket_path);
                        exit(code);
                    }
                }
            }
        });
    }

    #[allow(unused_must_use)]
    pub fn on_signal(&self, signal: Signal, action: Action) {
        let socket_path = self.socket_path.to_string();
        match signal {
            Signal::CtrlC => {
                ctrlc::set_handler(move || {
                    // TODO: handler result
                    match action {
                        Action::Terminate => {
                            _ = Self::terminate(&socket_path);
                        }
                        Action::TerminateExit(code) => {
                            _ = Self::terminate(&socket_path);
                            exit(code)
                        }
                    };
                });
            }
        }
    }

    pub fn accept_connections(&self) -> IncomingSocketConnection {
        IncomingSocketConnection { sc: self }
    }
}

impl<'sc> Iterator for IncomingSocketConnection<'sc> {
    type Item = std::io::Result<SocketConnection>;

    fn next(&mut self) -> Option<std::io::Result<SocketConnection>> {
        let Some(listener) = &self.sc.listener else {
            return Some(Err(SocketError::new(
                SocketErrorKind::SocketConnectionMissingListener,
                "Field `listener` missing from struct IncomingSocketConnection",
                None,
            )
            .into()));
        };
        Some(listener.accept().map(|s| SocketConnection::new(s.0)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}
