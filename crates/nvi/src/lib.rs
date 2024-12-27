mod client;
mod connect;
mod nvim_api;
mod service;

pub mod cmd;
pub mod diagnostics;
pub mod error;
pub mod opts;
pub mod test;
pub mod types;

pub use connect::*;
pub use nvi_macros::*;
pub use nvim_api::*;
pub use service::*;

pub use client::Client;
pub use mrpc::Value;
// AutocmdEvent is special, because it's used in the user event API
pub use types::AutocmdEvent;
