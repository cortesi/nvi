mod client;
mod connect;
mod run;
mod service;

pub mod error;
pub mod nvim_api;
pub mod opts;
pub mod test;
pub mod types;

pub use client::Client;
pub use connect::*;
pub use msgpack_rpc::Value;
pub use run::*;
pub use service::*;
