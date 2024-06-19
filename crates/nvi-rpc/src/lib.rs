//! This crate provides facilities to use the `MessagePack` remote procedure call system
//! (`MessagePack-RPC`) in Rust.

mod codec;
mod endpoint;
mod errors;
pub mod message;

pub use crate::endpoint::{serve, Ack, Client, Endpoint, Response, Service, ServiceWithClient};
pub use crate::errors::DecodeError;
pub use rmpv::{Integer, Utf8String, Value};
