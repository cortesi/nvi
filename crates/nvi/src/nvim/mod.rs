//! An interface to the NeoVim API. This module hews closely to the NeoVim API itself, without
//! adding higher-level abstractions. It consists of an API auto-generated from the NeoVim
//! msgpack-rpc interface definitions, with some additional types and helpers to improve type
//! safety and ergonomics.

mod api;
pub mod opts;
pub mod types;

pub use api::NvimApi;
