/// To generate these values, I inspect the output of `nvim --api-info`, like so:
///  
/// ```sh
// nvim --api-info | msgpack2json | jq
/// ```
use regex::Regex;
use serde::de::IntoDeserializer;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(remote = "Type")]
pub enum Type {
    Array,
    ArrayOf {
        typ: Box<Type>,
        length: Option<u32>,
    },
    Boolean,
    Buffer,
    Dictionary,
    Float,
    Function,
    Integer,
    LuaRef,
    Object,
    String,
    Tabpage,
    #[serde(rename = "void")]
    Void,
    Window,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Array => write!(f, "array"),
            Type::ArrayOf { typ, length } => {
                write!(f, "arrayOf({}", typ)?;
                if let Some(length) = length {
                    write!(f, ", {}", length)?;
                }
                write!(f, ")")
            }
            Type::Boolean => write!(f, "bool"),
            Type::Buffer => write!(f, "buffer"),
            Type::Dictionary => write!(f, "dict"),
            Type::Float => write!(f, "float"),
            Type::Function => write!(f, "func"),
            Type::Integer => write!(f, "int"),
            Type::LuaRef => write!(f, "luaRef"),
            Type::Object => write!(f, "object"),
            Type::String => write!(f, "string"),
            Type::Tabpage => write!(f, "tabpage"),
            Type::Void => write!(f, "void"),
            Type::Window => write!(f, "window"),
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
            Ok(Type::ArrayOf {
                typ: Box::new(Type::deserialize(
                    captures.get(1).unwrap().as_str().into_deserializer(),
                )?),
                length: captures.get(2).map(|m| m.as_str().parse().unwrap()),
            })
        } else {
            Type::deserialize(s.into_deserializer())
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ExtType {
    pub id: u32,
    pub prefix: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Parameter(pub Type, pub String);

#[derive(Debug, PartialEq, Deserialize)]
pub struct Function {
    pub method: bool,
    pub name: String,
    pub since: u32,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub deprecated_since: Option<u32>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct UIEvent {
    pub name: String,
    pub since: u32,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: String,
    pub prerelease: bool,
    pub api_level: u32,
    pub api_compatible: u32,
    // This suddenly switched from bool to now returning null - I'm assuming it's an Option<bool>
    pub api_prerelease: Option<bool>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Api {
    pub version: Version,
    pub functions: Vec<Function>,
    pub ui_events: Vec<UIEvent>,
    pub ui_options: Vec<String>,
    pub error_types: HashMap<String, ExtType>,
    pub types: HashMap<String, ExtType>,
}
