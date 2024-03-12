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
    id: u32,
    prefix: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Parameter(Type, String);

#[derive(Debug, PartialEq, Deserialize)]
pub struct Function {
    method: bool,
    name: String,
    since: u32,
    parameters: Vec<Parameter>,
    return_type: Type,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct UIEvent {
    name: String,
    since: u32,
    parameters: Vec<Parameter>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    build: String,
    prerelease: bool,
    api_level: u32,
    api_compatible: u32,
    api_prerelease: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Api {
    version: Version,
    functions: Vec<Function>,
    ui_events: Vec<UIEvent>,
    ui_options: Vec<String>,
    error_types: HashMap<String, ExtType>,
    types: HashMap<String, ExtType>,
}
