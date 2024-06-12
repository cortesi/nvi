//! This module contains the options structures for methods in the generated API.
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

/// A group specification, used in many command options. Groups can be specified as either a string
/// name, or as a numeric ID.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Group {
    Name(String),
    Id(u64),
}

/// Options for `nvim_create_autocmd` method
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct CreateAutocmdOpts {
    /// Autocommand group name or ID to match against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct ExecAutocmdsOpts {
    /// Autocommand group name or ID to match against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use rmpv::Value;
    use serde_rmpv::{from_value, to_value};

    #[test]
    fn test_serialize_group() {
        let group = Group::Name("test".to_string());
        let serialized = to_value(&group).unwrap();
        assert_eq!(serialized, Value::String("test".to_string().into()));
        let g2: Group = from_value(&serialized).unwrap();
        assert_eq!(group, g2);

        let group = Group::Id(5);
        let serialized = to_value(&group).unwrap();
        assert_eq!(serialized, Value::Integer(5.into()));
        let g2: Group = from_value(&serialized).unwrap();
        assert_eq!(group, g2);
    }
}
