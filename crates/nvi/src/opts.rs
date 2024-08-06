//! This module contains the optional argument structures for methods in the generated API. These
//! are the final arguments in API functions named `opts`. They are not included in the rendered
//! protocol description, so we have to write them by hand.

use crate::types;
use derive_setters::*;
use serde_derive::{Deserialize, Serialize};

/// Options for `nvim_buf_delete` method
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct BufDelete {
    /// Force deletion and ignore unsaved changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Unloaded only, do not delete the buffer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unload: Option<bool>,
}

/// Options for `nvim_create_autocmd` method
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct CreateAutocmd {
    /// Autocommand group name or ID to match against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<types::Group>,
    /// Pattern to match literally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Vec<String>>,
    /// Buffer number for buffer-local autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<u64>,
    /// Description for docs and troubleshooting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Vimscript function name to call when the event is triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    /// Vim command to execute when the event is triggered. Can't be used with callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Run the command only once
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,
    /// Run nested autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<bool>,
}

/// Options for `nvim_exec_autocmds` function
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct ExecAutocmds {
    /// Autocommand group name or ID to match against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<types::Group>,
    /// Pattern to match literally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Vec<String>>,
    /// Buffer number for buffer-local autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<u64>,
    /// Process the modeline after the autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modeline: Option<bool>,
    /// Data to send to event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

/// Options for `nvim_exec_autocmds` function
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct SetOptionValue {
    /// "global" or "local", analogous to ":setglobal" an ":setlocal"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Window ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win: Option<types::Window>,
    /// Buffer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf: Option<types::Buffer>,
}

/// Options for `nvim_clear_autocmds` method
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct ClearAutocmds {
    /// Event or events to clear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<String>>,

    /// Pattern or patterns to match exactly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Vec<String>>,

    /// Buffer number for buffer-local autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<types::Buffer>,

    /// Autocommand group name or ID to match against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<types::Group>,
}
