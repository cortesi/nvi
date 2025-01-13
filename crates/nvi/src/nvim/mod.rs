//! An interface to the NeoVim API. This module hews closely to the NeoVim API itself, without
//! adding higher-level functionality.
//!
//! It contains an API auto-generated from the NeoVim msgpack-rpc interface definitions, with
//! some additional types and helpers to improve type safety and ergonomics.
//!
//! Where the msgpack-rpc interface has gaps, like the diagnostic API, we provide a low-level
//! interface that interacts with NeoVim through a Lua bridge.

mod api;
pub mod diagnostics;
pub mod opts;
pub mod types;

pub use api::NvimApi;
