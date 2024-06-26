use rmpv::Value;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] rmpv::encode::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmpv::decode::Error),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Service error: {0}")]
    Service(ServiceError),
}

/// Represents an error that occurred during the execution of an RPC service method.
/// It consists of a name, which identifies the type of error, and a value, which
/// can contain additional error details. This error type is used to convey
/// service-specific errors back to the client. When sent over the RPC protocol,
/// this error will be serialized into a map with "name" and "value" keys.
#[derive(Error, Debug)]
pub struct ServiceError {
    pub name: String,
    pub value: Value,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Service error {}: {:?}", self.name, self.value)
    }
}

impl From<ServiceError> for Value {
    fn from(error: ServiceError) -> Self {
        Value::Map(vec![
            (
                Value::String("name".into()),
                Value::String(error.name.into()),
            ),
            (Value::String("value".into()), error.value),
        ])
    }
}

pub type Result<T> = std::result::Result<T, RpcError>;
