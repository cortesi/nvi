/// To generate these values, I inspect the output of `nvim --api-info`, like so:
///  
/// ```sh
// nvim --api-info | msgpack2json | jq
/// ```
use std::{collections::HashMap, fmt, process::Command};

use anyhow::Result;
use regex::Regex;
use rmp_serde as rmps;
use serde::de::IntoDeserializer;
use serde_derive::Deserialize;

/// The type of a Neovim API parameter or return value
#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(remote = "Type")]
pub enum Type {
    /// An array
    Array,
    /// An array of a specific type
    ArrayOf {
        /// The type of the array elements
        typ: Box<Self>,
        /// The length of the array
        length: Option<u32>,
    },
    /// A boolean
    Boolean,
    /// A buffer
    Buffer,
    /// A dictionary
    Dictionary,
    /// A float
    Float,
    /// A function
    Function,
    /// An integer
    Integer,
    /// A Lua reference
    LuaRef,
    /// A dictionary (alias)
    Dict,
    /// An object
    Object,
    /// A string
    String,
    /// A tabpage
    Tabpage,
    /// Void
    #[serde(rename = "void")]
    Void,
    /// A window
    Window,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array => write!(f, "array"),
            Self::ArrayOf { typ, length } => {
                write!(f, "arrayOf({typ}")?;
                if let Some(length) = length {
                    write!(f, ", {length}")?;
                }
                write!(f, ")")
            }
            Self::Boolean => write!(f, "bool"),
            Self::Buffer => write!(f, "buffer"),
            Self::Dictionary => write!(f, "dict"),
            Self::Float => write!(f, "float"),
            Self::Function => write!(f, "func"),
            Self::Integer => write!(f, "int"),
            Self::LuaRef => write!(f, "luaRef"),
            Self::Dict => write!(f, "dict"),
            Self::Object => write!(f, "object"),
            Self::String => write!(f, "string"),
            Self::Tabpage => write!(f, "tabpage"),
            Self::Void => write!(f, "void"),
            Self::Window => write!(f, "window"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let re = Regex::new(r"^ArrayOf\((\w+)(?:, (\d+))?\)$").unwrap();
        let s = String::deserialize(deserializer)?;
        if let Some(captures) = re.captures(&s) {
            Ok(Self::ArrayOf {
                typ: Box::new(Self::deserialize(
                    captures.get(1).unwrap().as_str().into_deserializer(),
                )?),
                length: captures.get(2).map(|m| m.as_str().parse().unwrap()),
            })
        } else {
            Self::deserialize(s.into_deserializer())
        }
    }
}

/// An extension type
#[derive(Debug, PartialEq, Deserialize)]
pub struct ExtType {
    /// The ID of the extension type
    pub id: u32,
    /// The prefix of the extension type
    pub prefix: Option<String>,
}

/// A parameter to a function
#[derive(Debug, PartialEq, Deserialize)]
pub struct Parameter(pub Type, pub String);

/// A function definition
#[derive(Debug, PartialEq, Deserialize)]
pub struct Function {
    /// Whether the function is a method
    pub method: bool,
    /// The name of the function
    pub name: String,
    /// The API level since the function was introduced
    pub since: u32,
    /// The parameters to the function
    pub parameters: Vec<Parameter>,
    /// The return type of the function
    pub return_type: Type,
    /// The API level since the function was deprecated
    pub deprecated_since: Option<u32>,
}

/// A UI event definition
#[derive(Debug, PartialEq, Deserialize)]
pub struct UIEvent {
    /// The name of the event
    pub name: String,
    /// The API level since the event was introduced
    pub since: u32,
    /// The parameters to the event
    pub parameters: Vec<Parameter>,
}

/// A version definition
#[derive(Debug, PartialEq, Deserialize)]
pub struct Version {
    /// The major version
    pub major: u32,
    /// The minor version
    pub minor: u32,
    /// The patch version
    pub patch: u32,
    /// The build version
    pub build: String,
    /// Whether the version is a prerelease
    pub prerelease: bool,
    /// The API level
    pub api_level: u32,
    /// The API compatibility level
    pub api_compatible: u32,
    // This suddenly switched from bool to now returning null - I'm assuming it's an Option<bool>
    /// Whether the API is a prerelease
    pub api_prerelease: Option<bool>,
}

/// An API definition
#[derive(Debug, PartialEq, Deserialize)]
pub struct Api {
    /// The version of the API
    pub version: Version,
    /// The functions in the API
    pub functions: Vec<Function>,
    /// The UI events in the API
    pub ui_events: Vec<UIEvent>,
    /// The UI options in the API
    pub ui_options: Vec<String>,
    /// The error types in the API
    pub error_types: HashMap<String, ExtType>,
    /// The types in the API
    pub types: HashMap<String, ExtType>,
}

/// Get the Neovim API definition
pub fn get_api() -> Result<Api> {
    let output = Command::new("nvim").arg("--api-info").output()?;
    Ok(rmps::from_slice(&output.stdout)?)
}
