/* A compendium of types for working with the Neovim msgrpc API */
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes, NoneAsEmptyString};

pub const BUFFER_EXT_TYPE: i8 = 0;
pub const WINDOW_EXT_TYPE: i8 = 1;
pub const TABPAGE_EXT_TYPE: i8 = 2;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "_ExtStruct")]
pub struct Buffer(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Buffer {
    pub fn current() -> Self {
        Buffer((BUFFER_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "_ExtStruct")]
pub struct Window(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Window {
    pub fn current() -> Self {
        Window((WINDOW_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "_ExtStruct")]
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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AutocmdEvent {
    pub id: u64,
    pub event: Event,
    pub group: Option<u64>,
    pub matches: Option<Vec<String>>,
    pub buf: u64,
    pub file: String,
    pub data: Option<crate::Value>,
}

/// Autocommand events. See here for documentation: https://neovim.io/doc/user/autocmd.html#autocmd-events
#[derive(
    Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString, Deserialize, Serialize,
)]
pub enum Event {
    BufAdd,
    BufDelete,
    BufEnter,
    BufFilePost,
    BufFilePre,
    BufHidden,
    BufLeave,
    BufNew,
    BufModifiedSet,
    BufNewFile,
    BufRead,
    BufReadPost,
    BufWinLeave,
    BufWipeout,
    BufWrite,
    BufWritePre,
    BufWritePost,
    BufWriteCmd,
    ChanInfo,
    ChanOpen,
    CmdUndefined,
    CmdlineChanged,
    CmdlineEnter,
    CmdlineLeave,
    CmdwinEnter,
    CmdwinLeave,
    ColorScheme,
    ColorSchemePre,
    CompleteChanged,
    CompleteDonePre,
    CompleteDone,
    CursorHold,
    CursorHoldI,
    CursorMoved,
    CursorMovedI,
    DiffUpdated,
    DirChanged,
    DirChangedPre,
    ExitPre,
    FileAppendCmd,
    FileAppendPost,
    FileAppendPre,
    FileChangedRO,
    FileChangedShell,
    FileChangedShellPost,
    FileReadCmd,
    FileReadPost,
    FileReadPre,
    FileType,
    FileWriteCmd,
    FileWritePost,
    FileWritePre,
    FilterReadPost,
    FilterReadPre,
    FilterWritePost,
    FilterWritePre,
    FocusGained,
    FocusLost,
    FuncUndefined,
    UIEnter,
    UILeave,
    InsertChange,
    InsertCharPre,
    InsertEnter,
    InsertLeavePre,
    InsertLeave,
    MenuPopup,
    ModeChanged,
    OptionSet,
    QuickFixCmdPre,
    QuickFixCmdPost,
    QuitPre,
    RemoteReply,
    SearchWrapped,
    RecordingEnter,
    RecordingLeave,
    SafeState,
    SessionLoadPost,
    SessionWritePost,
    ShellCmdPost,
    Signal,
    ShellFilterPost,
    SourcePre,
    SourcePost,
    SourceCmd,
    SpellFileMissing,
    StdinReadPost,
    StdinReadPre,
    SwapExists,
    Syntax,
    TabEnter,
    TabLeave,
    TabNew,
    TabNewEntered,
    TabClosed,
    TermOpen,
    TermEnter,
    TermLeave,
    TermClose,
    TermRequest,
    TermResponse,
    TextChanged,
    TextChangedI,
    TextChangedP,
    TextChangedT,
    TextYankPost,
    User,
    UserGettingBored,
    VimEnter,
    VimLeave,
    VimLeavePre,
    VimResized,
    VimResume,
    VimSuspend,
    WinClosed,
    WinEnter,
    WinLeave,
    WinNew,
    WinScrolled,
    WinResized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl LogLevel {
    pub fn to_u64(&self) -> u64 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }
}
#[derive(
    Debug, Clone, Deserialize, Serialize, PartialEq, Eq, strum::EnumString, strum::Display,
)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Relative {
    Editor,
    Win,
    Cursor,
    Mouse,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Anchor {
    NW,
    NE,
    SW,
    SE,
}

/// A group specification, used in many command options. Groups can be specified as either a string
/// name, or as a numeric ID.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Text {
    Plain(String),
    Id(Vec<(String, String)>),
}

/// A group specification, used in many command options. Groups can be specified as either a string
/// name, or as a numeric ID.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Border {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
    Array(Vec<String>),
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct WindowConf {
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative: Option<Relative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufpos: Option<(u64, u64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focusable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zinc: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Border>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_pos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noautocmd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmpv::Value;
    use serde_rmpv::from_value;

    #[test]
    fn test_deser_event() {
        let v: Event = from_value(&Value::from("User")).unwrap();
        assert_eq!(v, Event::User);
    }

    #[test]
    fn test_deser_autocmdevent() {
        let evt = Value::Map(vec![
            (Value::from("id"), Value::from(1)),
            (Value::from("event"), Value::from("User")),
            (Value::from("group"), Value::from(1)),
            (
                Value::from("matches"),
                Value::Array(vec![Value::from("*.rs")]),
            ),
            (Value::from("buf"), Value::from(1)),
            (Value::from("file"), Value::from("file")),
            (Value::from("data"), Value::from("data")),
        ]);

        let expected = AutocmdEvent {
            id: 1,
            event: Event::User,
            group: Some(1),
            matches: Some(vec!["*.rs".to_string()]),
            buf: 1,
            file: "file".into(),
            data: Some(rmpv::Value::from("data")),
        };

        let ret: AutocmdEvent = from_value(&evt).unwrap();
        assert_eq!(ret, expected);
    }
}
