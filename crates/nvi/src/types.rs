/* A compendium of types for working with the Neovim msgrpc API */

use crate::error::{Error, Result};
use rmpv::Value;

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

#[derive(Debug)]
pub struct ApiInfo {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub api_level: u64,
    pub api_compatible: u64,
    pub build: String,
    pub prerelease: bool,
}

impl TryFrom<Value> for ApiInfo {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        let v = get_map(&value, "version")?;
        Ok(ApiInfo {
            major: get_map(v, "major")?.clone().try_into()?,
            minor: get_map(v, "minor")?.clone().try_into()?,
            patch: get_map(v, "patch")?.clone().try_into()?,
            build: get_map(v, "build")?.clone().try_into()?,
            api_level: get_map(v, "api_level")?.clone().try_into()?,
            api_compatible: get_map(v, "api_compatible")?.clone().try_into()?,
            prerelease: get_map(v, "prerelease")?.clone().try_into()?,
        })
    }
}

/// Retrieve a value from a msgpack map by name
fn get_map<'a>(map: &'a Value, name: &str) -> Result<&'a Value> {
    try_get_map(map, name)?.ok_or(Error::Decode {
        msg: format!("Expected map key: {}", name),
    })
}

/// Try to retrieve a value from a msgpack map by name
fn try_get_map<'a>(map: &'a Value, name: &str) -> Result<Option<&'a Value>> {
    let map = map.as_map().ok_or(Error::Decode {
        msg: "Expected map".into(),
    })?;
    let n = Value::String(name.into());
    for (k, v) in map {
        if k == &n {
            return Ok(Some(v));
        }
    }
    Ok(None)
}

#[allow(dead_code)]
/// Utility to get keys from a map
fn map_keys(map: &Value) -> Result<Vec<String>> {
    let map = map.as_map().ok_or(Error::Decode {
        msg: "Expected map".into(),
    })?;
    let mut keys = Vec::new();
    for (k, _) in map {
        if k.is_str() {
            keys.push(k.as_str().unwrap().to_string());
        } else {
            return Err(Error::Decode {
                msg: "Expected string".into(),
            });
        }
    }
    Ok(keys)
}

pub(crate) fn nvim_get_api_info_return(ret: &Value) -> Result<(u64, ApiInfo)> {
    let arr = ret.as_array().ok_or(Error::Decode {
        msg: "Expected array".into(),
    })?;
    if arr.len() != 2 {
        return Err(Error::Decode {
            msg: "Expected array of length 2".into(),
        });
    }
    let chan = arr[0].clone().try_into()?;
    let api_info = arr[1].clone().try_into()?;
    Ok((chan, api_info))
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_api_info() {}
}
