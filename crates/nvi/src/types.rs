/* A compendium of types for working with the Neovim msgrpc API */

use crate::error::{Error, Result};
use rmpv::Value;
use serde_derive::Deserialize;

pub const BUFFER_EXT_TYPE: i8 = 0;
pub const WINDOW_EXT_TYPE: i8 = 1;
pub const TABPAGE_EXT_TYPE: i8 = 2;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
