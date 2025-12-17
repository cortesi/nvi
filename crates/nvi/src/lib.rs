//! The Nvi library.
#![allow(clippy::absolute_paths)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::let_underscore_must_use)]
#![allow(missing_docs)]

mod client;
mod process;
mod service;

pub mod cmd;
pub mod connect;
pub mod demo;
pub mod docs;
pub mod error;
pub mod highlights;
pub mod input;
pub mod lua;
pub mod nvim;
pub mod test;
pub mod ui;

// Re-export, because we use this in our derive code
#[doc(hidden)]
pub use async_trait;
pub use client::Client;
// Re-export consistent color names
pub use colornames::Color;
pub use macro_types;
pub use mrpc::Value;
pub use nvi_macros;
// AutocmdEvent is special, because it's used in the user event API
pub use nvim::types::AutocmdEvent;
#[doc(hidden)]
pub use serde_rmpv;
pub use service::*;
