/* A compendium of types for working with the Neovim msgrpc API */

use crate::error::{Error, Result};
use derive_builder::Builder;
use rmpv::Value;
use serde_derive::{Deserialize, Serialize};

pub const BUFFER_EXT_TYPE: i8 = 0;
pub const WINDOW_EXT_TYPE: i8 = 1;
pub const TABPAGE_EXT_TYPE: i8 = 2;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Buffer {
    pub(crate) data: Vec<u8>,
}

impl Buffer {
    pub fn current() -> Self {
        Buffer {
            data: vec![0, 0, 0, 0],
        }
    }

    // Create a Buffer from an rmpv::Value
    pub fn from_value(value: &Value) -> Result<Self> {
        if let Value::Ext(BUFFER_EXT_TYPE, ref data) = *value {
            Ok(Buffer { data: data.clone() })
        } else {
            Err(Error::Decode {
                msg: "Expected Buffer ext type".into(),
            })
        }
    }

    // Render the Buffer to an rmpv::Value
    pub fn to_value(&self) -> Value {
        Value::Ext(BUFFER_EXT_TYPE, self.data.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Window {
    pub(crate) data: Vec<u8>,
}

impl Window {
    pub fn current() -> Self {
        Window {
            data: vec![0, 0, 0, 0],
        }
    }

    // Create a Buffer from an rmpv::Value
    pub fn from_value(value: &Value) -> Result<Self> {
        if let Value::Ext(WINDOW_EXT_TYPE, ref data) = *value {
            Ok(Window { data: data.clone() })
        } else {
            Err(Error::Decode {
                msg: "Expected Window ext type".into(),
            })
        }
    }

    // Render the Buffer to an rmpv::Value
    pub fn to_value(&self) -> Value {
        Value::Ext(WINDOW_EXT_TYPE, self.data.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TabPage {
    pub(crate) data: Vec<u8>,
}

impl TabPage {
    pub fn current() -> Self {
        TabPage {
            data: vec![0, 0, 0, 0],
        }
    }

    // Create a Buffer from an rmpv::Value
    pub fn from_value(value: &Value) -> Result<Self> {
        if let Value::Ext(TABPAGE_EXT_TYPE, ref data) = *value {
            Ok(TabPage { data: data.clone() })
        } else {
            Err(Error::Decode {
                msg: "Expected TabPage ext type".into(),
            })
        }
    }

    // Render the Buffer to an rmpv::Value
    pub fn to_value(&self) -> Value {
        Value::Ext(TABPAGE_EXT_TYPE, self.data.clone())
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct APIVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub api_level: u64,
    pub api_compatible: u64,
    pub build: String,
    pub prerelease: bool,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ApiInfo {
    pub version: APIVersion,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Group {
    Name(String),
    Id(u64),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct CreateAutocmdOpts {
    pub group: Option<Group>,
    pub pattern: Vec<String>,
    pub buffer: Option<u64>,
    pub desc: Option<String>,
    pub callback: Option<String>,
    pub command: Option<String>,
    pub once: Option<bool>,
    pub nested: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
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
