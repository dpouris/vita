use std::result::Result as StdResult;

pub type Result<T, M> = StdResult<T, super::error::SocketError<M>>;
