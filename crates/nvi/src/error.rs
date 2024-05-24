pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("request timed out: {method:?}")]
    Timeout { method: String },
    #[error("decoding error: {msg:?}")]
    Decode { msg: String },
    #[error("encoding error: {msg:?}")]
    Encode { msg: String },
    #[error("io error: {msg:?}")]
    IO { msg: String },
}

impl From<rmp::encode::ValueWriteError> for Error {
    fn from(e: rmp::encode::ValueWriteError) -> Self {
        Error::Encode {
            msg: format!("{}", e),
        }
    }
}

impl From<rmp::decode::ValueReadError> for Error {
    fn from(e: rmp::decode::ValueReadError) -> Self {
        Error::Decode {
            msg: format!("{}", e),
        }
    }
}

impl From<rmp::decode::DecodeStringError<'_>> for Error {
    fn from(e: rmp::decode::DecodeStringError) -> Self {
        Error::Decode {
            msg: format!("{}", e),
        }
    }
}

impl From<rmpv::decode::Error> for Error {
    fn from(e: rmpv::decode::Error) -> Self {
        Error::Decode {
            msg: format!("{}", e),
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
        Error::IO {
            msg: format!("{}", e),
        }
    }
}
