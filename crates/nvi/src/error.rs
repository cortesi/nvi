//! Error handling for Nvi.
//!
//! This module provides the standard error types and Result type used throughout Nvi.

#![allow(clippy::absolute_paths)]

/// Standard Result type for Nvi operations, defaulting to the Nvi Error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Nvi standard error types
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error connecting to a Neovim instance
    #[error("connection error: {msg:}")]
    Connect { msg: String },
    /// Error decoding MessagePack data from Neovim
    #[error("decoding error: {msg:}")]
    Decode { msg: String },
    /// Error encoding MessagePack data for Neovim
    #[error("encoding error: {msg:}")]
    Encode { msg: String },
    /// File system or network I/O error
    #[error("io error: {msg:}")]
    IO { msg: String },
    /// Error returned from a Neovim RPC call
    #[error("remote error: {0:}")]
    RemoteError(rmpv::Value),
    /// Internal error in the Nvi library
    #[error("internal: {msg:}")]
    Internal { msg: String },
    /// An error caused by the user through invalid input
    #[error("{0}")]
    User(String),
}

impl From<serde_rmpv::Error> for Error {
    fn from(e: serde_rmpv::Error) -> Self {
        Self::Decode {
            msg: format!("serde: {e}"),
        }
    }
}

impl From<rmp::encode::ValueWriteError> for Error {
    fn from(e: rmp::encode::ValueWriteError) -> Self {
        Self::Encode {
            msg: format!("{e}"),
        }
    }
}

impl From<rmp::decode::ValueReadError> for Error {
    fn from(e: rmp::decode::ValueReadError) -> Self {
        Self::Decode {
            msg: format!("value read: {e}"),
        }
    }
}

impl From<rmp::decode::DecodeStringError<'_>> for Error {
    fn from(e: rmp::decode::DecodeStringError) -> Self {
        Self::Decode {
            msg: format!("rmp decode: {e}"),
        }
    }
}

impl From<rmpv::decode::Error> for Error {
    fn from(e: rmpv::decode::Error) -> Self {
        Self::Decode {
            msg: format!("rmpv decode: {e}"),
        }
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_e: std::convert::Infallible) -> Self {
        panic!("infallible")
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO {
            msg: format!("{e}"),
        }
    }
}

impl From<mrpc::RpcError> for Error {
    fn from(e: mrpc::RpcError) -> Self {
        match e {
            mrpc::RpcError::Service(e) => Self::RemoteError(e.value),
            mrpc::RpcError::Connect { source } => Self::Connect {
                msg: source.to_string(),
            },
            e => Self::Internal {
                msg: format!("{e:?}"),
            },
        }
    }
}
