use rmpv::decode;
use std::{error, fmt, io};

/// Error while decoding a sequence of bytes into a `MessagePack-RPC` message
#[derive(Debug)]
pub enum DecodeError {
    /// Some bytes are missing to decode a full msgpack value
    Truncated(io::Error),
    /// A byte sequence could not be decoded as a msgpack value, or this value is not a valid
    /// msgpack-rpc message.
    Invalid,
    /// The maximum recursion depth before [`Error::DepthLimitExceeded`] is returned.
    DepthLimitExceeded,
    /// An unknown IO error while reading a byte sequence
    UnknownIo(io::Error),
}

impl DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::Truncated(_) => "could not read enough bytes to decode a complete message",
            DecodeError::UnknownIo(_) => "Unknown IO error while decoding a message",
            DecodeError::Invalid => "the byte sequence is not a valid msgpack-rpc message",
            DecodeError::DepthLimitExceeded => {
                "The depth limit [`MAX_DEPTH`] of 1024 bytes were exceeded."
            }
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

impl error::Error for DecodeError {
    fn description(&self) -> &str {
        self.description()
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            DecodeError::UnknownIo(ref e) => Some(e),
            DecodeError::Truncated(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> DecodeError {
        log::error!("Got I/O error {err}");
        match err.kind() {
            io::ErrorKind::UnexpectedEof => DecodeError::Truncated(err),
            io::ErrorKind::Other => {
                if let Some(cause) = err.get_ref().unwrap().source() {
                    // XXX Allocating here sucks, but `description` is deprecated :(
                    //     Regardless, this depends on implementation details of rmpv, which isn't
                    //     a great idea in the first place.
                    if cause.to_string() == "type mismatch" {
                        return DecodeError::Invalid;
                    }
                }
                DecodeError::UnknownIo(err)
            }
            _ => DecodeError::UnknownIo(err),
        }
    }
}

impl From<decode::Error> for DecodeError {
    fn from(err: decode::Error) -> DecodeError {
        match err {
            decode::Error::InvalidMarkerRead(io_err) | decode::Error::InvalidDataRead(io_err) => {
                From::from(io_err)
            }
            decode::Error::DepthLimitExceeded => DecodeError::DepthLimitExceeded,
        }
    }
}
