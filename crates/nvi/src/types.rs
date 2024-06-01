/* A compendium of types for working with the Neovim msgrpc API */
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

pub const BUFFER_EXT_TYPE: i8 = 0;
pub const WINDOW_EXT_TYPE: i8 = 1;
pub const TABPAGE_EXT_TYPE: i8 = 2;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Buffer(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Buffer {
    pub fn current() -> Self {
        Buffer((BUFFER_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Window(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Window {
    pub fn current() -> Self {
        Window((WINDOW_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TabPage(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl TabPage {
    pub fn current() -> Self {
        TabPage((TABPAGE_EXT_TYPE, vec![0, 0, 0, 0]))
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
