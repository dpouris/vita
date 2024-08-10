use std::{
    error::Error,
    fmt::{Debug, Display},
    io::ErrorKind,
};

macro_rules! from_io_err {
    ($kind:ident, $value:ident) => {
        $crate::error::SocketError::new(
            SocketErrorKind::$kind,
            $value.to_string(),
            Some($value.into()),
        )
    };
}

#[derive(Debug)]
pub struct ErrorWhere {
    line: u32,
    column: u32,
    file: &'static str,
}

impl ErrorWhere {
    pub fn new(line: u32, column: u32, file: &'static str) -> Self {
        Self { column, line, file }
    }
}

impl Display for ErrorWhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.file,
            line = self.line,
            column = self.column
        )
    }
}

#[derive(Debug)]
pub struct SocketError<T>
where
    T: Into<Box<dyn std::error::Error + Send + Sync>> + Display + Debug,
{
    pub error_location: Option<ErrorWhere>,
    kind: SocketErrorKind,
    message: T,
    source: Option<Box<dyn Error>>,
}

#[derive(Debug)]
pub enum SocketErrorKind {
    SocketBind,
    SocketTermination,
    SocketConnect,
    SocketStreamIO,
    SocketStreamEmpty,
    SocketConnectionMissingListener,
    SocketAddressInUse,
    SocketInvalidPath,
    SocketConnectionClosed,
    Other,
}

pub enum BytesError {
    InvalidLength,
}

impl<T> SocketError<T>
where
    T: Into<Box<dyn std::error::Error + Send + Sync>> + Display + Debug,
{
    pub fn new(kind: SocketErrorKind, message: T, source: Option<Box<dyn Error>>) -> Self {
        Self {
            kind,
            message,
            source,
            error_location: None,
        }
    }

    pub fn err_where(mut self, location: ErrorWhere) -> Self {
        self.error_location = Some(location);
        self
    }

    pub fn kind(&self) -> &SocketErrorKind {
        &self.kind
    }
}
impl SocketError<String> {
    pub fn log(&self) {
        eprintln!("{self}");
    }
}

impl<T> From<SocketError<T>> for std::io::Error
where
    T: Into<Box<dyn std::error::Error + Send + Sync>> + Display + Debug,
{
    fn from(value: SocketError<T>) -> Self {
        match value.kind {
            SocketErrorKind::SocketConnectionMissingListener => Self::new(
                ErrorKind::Other,
                "Field `listener: UnixListener` missing in struct SockerClient",
            ),
            SocketErrorKind::SocketStreamEmpty => Self::new(ErrorKind::Other, "Empty stream"),
            SocketErrorKind::SocketInvalidPath => Self::new(ErrorKind::NotFound, value.message),
            SocketErrorKind::SocketBind => Self::new(ErrorKind::AlreadyExists, value.message),
            SocketErrorKind::SocketConnect => {
                Self::new(ErrorKind::ConnectionAborted, value.message)
            }
            SocketErrorKind::SocketTermination => Self::new(ErrorKind::Other, value.message),
            SocketErrorKind::SocketStreamIO => Self::new(ErrorKind::Interrupted, value.message),
            SocketErrorKind::SocketAddressInUse => Self::new(ErrorKind::AddrInUse, value.message),
            SocketErrorKind::SocketConnectionClosed => {
                Self::new(ErrorKind::BrokenPipe, value.message)
            }
            SocketErrorKind::Other => Self::new(ErrorKind::Interrupted, value.message),
        }
    }
}

impl From<std::io::Error> for SocketError<String> {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::Other => from_io_err!(SocketConnectionMissingListener, value),
            ErrorKind::InvalidData => from_io_err!(SocketStreamEmpty, value),
            ErrorKind::AlreadyExists => from_io_err!(SocketBind, value),
            ErrorKind::ConnectionAborted => from_io_err!(SocketConnect, value),
            ErrorKind::NotFound => from_io_err!(SocketInvalidPath, value),
            ErrorKind::PermissionDenied => from_io_err!(SocketTermination, value),
            ErrorKind::Interrupted => from_io_err!(SocketStreamIO, value),
            ErrorKind::AddrInUse => from_io_err!(SocketAddressInUse, value),
            ErrorKind::BrokenPipe => from_io_err!(SocketConnectionClosed, value),
            _ => from_io_err!(Other, value),
        }
    }
}

impl From<BytesError> for SocketError<String> {
    fn from(value: BytesError) -> Self {
        match value {
            BytesError::InvalidLength => SocketError::new(
                SocketErrorKind::SocketStreamIO,
                "Invalid byte liength".to_string(),
                None,
            ),
        }
    }
}

impl Display for SocketErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = match self {
            Self::SocketAddressInUse => "SocketAddressInUse",
            Self::Other => "Other",
            Self::SocketConnectionClosed => "SocketConnectionClosed",
            Self::SocketBind => "SocketBind",
            Self::SocketConnect => "SocketConnect",
            Self::SocketConnectionMissingListener => "SocketConnectionMissingListener",
            Self::SocketStreamEmpty => "SocketStreamEmpty",
            Self::SocketStreamIO => "SocketStreamIO",
            Self::SocketTermination => "SocketTermination",
            Self::SocketInvalidPath => "SocketInvalidPath",
        };

        write!(f, "{tag}")
    }
}

impl Display for SocketError<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut err_msg = String::new();

        if let Some(ref where_err) = self.error_location {
            err_msg = format!("Error: On {where_err}\n");
        };

        err_msg.push_str(
            format!(
                "{kind}: {message}",
                kind = self.kind,
                message = self.message
            )
            .as_str(),
        );

        write!(f, "{err_msg}")
    }
}

impl Error for SocketError<String> {
    fn cause(&self) -> Option<&dyn Error> {
        // TODO: complete this
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
