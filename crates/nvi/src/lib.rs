mod client;
mod connect;
mod service;

pub mod cmd;
pub mod demo;
pub mod diagnostics;
pub mod error;
pub mod highlights;
pub mod input;
pub mod lua;
pub mod nvim;
pub mod test;
pub mod ui;

pub use connect::*;
pub use macro_types;
pub use nvi_macros;
pub use service::*;

pub use client::Client;
pub use mrpc::Value;
// AutocmdEvent is special, because it's used in the user event API
pub use nvim::types::AutocmdEvent;

// Re-export, because we use this in our derive code
#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use serde_rmpv;
